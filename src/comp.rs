// Purpose: Contains structs and functions for the simulation components.

use ordered_float::OrderedFloat;
use bevy::prelude::Resource;
use serde::Deserialize;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

// World struct contains all the roads and vehicles in the simulation.
#[derive(Resource)]
pub struct World{
    pub roads: Vec<Road>,
    pub vehicles: Vec<Vehicle>,
}

// Implement the Display trait for the World struct.
impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Roads: {:?}\nVehicles:{:?})", self.roads, self.vehicles)
    }
}
// Deserialize the JSON data into corresponding structs
#[derive(Deserialize)]
struct RoadData {
    from: [f32; 3],
    to: [f32; 3],
    lanes: u8,
    speed_limit: f32,
    from_road: Vec<usize>,
    to_road: Vec<usize>,
    end_speed_limit: f32,
}

#[derive(Deserialize)]
struct VehicleData {
    position: f32,
    velocity: f32,
    acceleration: f32,
    break_deceleration: f32,
    on_road: usize,
    watch_distance: f32,
    destination: usize,
    destination_position: f32,
}

#[derive(Deserialize)]
struct WorldData {
    roads: Vec<RoadData>,
    vehicles: Vec<VehicleData>,
}


impl World{
    fn new() -> World{
        World{
            roads: Vec::new(),
            vehicles: Vec::new(),
        }
    }
    pub fn add_vehicle(&mut self,position:f32,velocity:f32,acceleration:f32,break_decceleration:f32,on_road:usize,watch_distance:f32,destination:usize,destination_position:f32){
        let mut vehicle = Vehicle{
            position: position,
            velocity: velocity,
            acceleration: acceleration,
            break_decceleration: break_decceleration,
            on_road: on_road,
            watch_distance: watch_distance,
            destination: destination,
            destination_position: destination_position,
            path: Vec::new()
        };
        
        vehicle.path = self.find_shortest_path(vehicle.on_road, vehicle.destination);
        vehicle.path.remove(0);
        println!("Path: {:?}", vehicle.path);
        self.vehicles.push(vehicle);
    }
    pub fn add_road(&mut self,from:(f32,f32,f32),to:(f32,f32,f32),lanes:u8,speed_limit:f32,from_road:Vec<usize>,to_road:Vec<usize>,end_speed_limit:f32){
        let mut road = Road{
            from: from,
            to: to,
            length: OrderedFloat(((to.0-from.0).powi(2) + (to.1-from.1).powi(2) + (to.2-from.2).powi(2)).sqrt()),
            lanes: lanes,
            speed_limit: speed_limit,
            from_road: from_road,
            to_road: to_road,
            obstacle_map: HashMap::new(),
            end_speed_limit: end_speed_limit
        };

        //.x0 is for road ends
        road.obstacle_map.insert(OrderedFloat((road.length*10.0).round()/10.0),road.end_speed_limit);
        self.roads.push(road);
    }
    pub fn reset(&mut self){
        self.roads.clear();
        self.vehicles.clear();
    }
    pub fn load_json(&mut self,contents:String){
        self.reset();
        let world_data: WorldData = serde_json::from_str(&contents).expect("Failed to deserialize JSON data.");
        // Add roads from the JSON data
        for road_data in world_data.roads {
            self.add_road(
                (road_data.from[0], road_data.from[1], road_data.from[2]),
                (road_data.to[0], road_data.to[1], road_data.to[2]),
                road_data.lanes,
                road_data.speed_limit,
                road_data.from_road,
                road_data.to_road,
                road_data.end_speed_limit,
            );
        }

        // Add vehicles from the JSON data
        for vehicle_data in world_data.vehicles {
            self.add_vehicle(
                vehicle_data.position,
                vehicle_data.velocity,
                vehicle_data.acceleration,
                vehicle_data.break_deceleration,
                vehicle_data.on_road,
                vehicle_data.watch_distance,
                vehicle_data.destination,
                vehicle_data.destination_position,
            );
        }
    }

    // Helper function to get adjacent roads for a given road index
    fn get_adjacent_roads(&self, road_index: usize) -> &[usize] {
        if road_index < self.roads.len() {
            let road = &self.roads[road_index];
            &road.to_road
        } else {
            &[]
        }
    }

    // Helper function to get the distance between two roads
    fn get_distance_between_roads(&self, road_index_1: usize, road_index_2: usize) -> f32 {
        if road_index_1 < self.roads.len() && road_index_2 < self.roads.len() {
            let road_1 = &self.roads[road_index_1];
            let road_2 = &self.roads[road_index_2];
            // For simplicity, assume 3D Euclidean distance between roads' end points
            ((road_1.to.0 - road_2.from.0).powi(2) + (road_1.to.1 - road_2.from.1).powi(2)).sqrt()
        } else {
            f32::INFINITY
        }
    }

    // Heuristic function to estimate the cost from a given road to the destination road.
    fn heuristic(&self, road_index: usize, destination_road: usize) -> OrderedFloat<f32> {
        OrderedFloat(self.get_distance_between_roads(road_index, destination_road))
    }

    // Function to find the shortest path using A* algorithm
    fn find_shortest_path(&self, start_road: usize, destination_road: usize) -> Vec<usize> {
        let mut distances = HashMap::new();
        let mut previous = HashMap::new();
        let mut queue = BinaryHeap::new();

        distances.insert(start_road, OrderedFloat(0.0));
        queue.push(Reverse((OrderedFloat(0.0) + self.heuristic(start_road, destination_road), start_road)));

        while let Some(Reverse((current_distance, current_road))) = queue.pop() {
            if current_road == destination_road {
                break;
            }

            for &next_road in self.get_adjacent_roads(current_road) {
                let distance_to_next = self.get_distance_between_roads(current_road, next_road);
                let total_distance = current_distance - self.heuristic(current_road, destination_road) + OrderedFloat(distance_to_next);

                if !distances.contains_key(&next_road) || total_distance < *distances.get(&next_road).unwrap() {
                    distances.insert(next_road, total_distance);
                    previous.insert(next_road, current_road);
                    queue.push(Reverse((total_distance + self.heuristic(next_road, destination_road), next_road)));
                }
            }
        }

        // Reconstruct the path
        let mut path = Vec::new();
        let mut current_road = destination_road;
        while let Some(&prev) = previous.get(&current_road) {
            path.push(current_road);
            current_road = prev;
        }
        path.push(start_road);
        path.reverse();
        path
    }


}

// Road struct contains the length, number of lanes, and speed limit of a road.}
#[derive(Debug)]
pub struct Road{
    pub from: (f32,f32,f32),
    pub to: (f32,f32,f32),
    pub length: OrderedFloat<f32>,
    pub lanes: u8,
    pub speed_limit: f32,
    pub from_road: Vec<usize>,
    pub to_road: Vec<usize>,
    pub obstacle_map: HashMap<OrderedFloat<f32>,f32>,
    pub end_speed_limit: f32
}

// Implement the Display trait for the Road struct.
impl std::fmt::Display for Road {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Length: {}\nLanes: {}\nSpeed Limit: {}", self.length, self.lanes, self.speed_limit)
    }
}


// Vehicle struct contains the position, velocity, and acceleration of a vehicle.
#[derive(Debug)]
pub struct Vehicle{
    pub position: f32,
    pub velocity: f32,
    pub acceleration: f32,
    pub break_decceleration: f32,
    pub on_road: usize,
    pub watch_distance: f32,
    pub destination: usize,
    pub destination_position: f32,
    pub path: Vec<usize>
}

// Implement the Display trait for the Vehicle struct.
impl std::fmt::Display for Vehicle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Position: {}\nVelocity: {}\nAcceleration: {}", self.position, self.velocity, self.acceleration)
    }
}



// Create a world with roads and vehicles.
// Returns a World struct.
pub fn sample_world() -> World{
    let mut world = World::new();
    world.add_road((0.0,10.0,0.0),(500.0,10.0,0.0),1,100.0,vec![0],vec![1],10.0);
    world.add_road((500.0,-10.0,0.0),(0.0,-10.0,0.0),1,100.0,vec![1],vec![0],10.0);
    world.add_vehicle(0.0,0.0,5.0,-10.0,0,200.0,1,250.0);
    world.add_vehicle(0.0,0.0,4.0,-7.0,1,250.0,0,311.0);
    return world;
}
