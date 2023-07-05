// Purpose: Contains structs and functions for the simulation components.
use std::collections::HashMap;
use ordered_float::OrderedFloat;

// World struct contains all the roads and vehicles in the simulation.
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

impl World{
    fn new() -> World{
        World{
            roads: Vec::new(),
            vehicles: Vec::new(),
        }
    }
    pub fn add_vehicle(&mut self,position:f32,velocity:f32,acceleration:f32,break_decceleration:f32,on_road:usize,watch_distance:f32,destination:usize,destination_position:f32){
        let vehicle = Vehicle{
            position: position,
            velocity: velocity,
            acceleration: acceleration,
            break_decceleration: break_decceleration,
            on_road: on_road,
            watch_distance: watch_distance,
            destination: destination,
            destination_position: destination_position
        };
        
        self.vehicles.push(vehicle);
    }
    pub fn add_road(&mut self,from:(f32,f32,f32),to:(f32,f32,f32),lanes:u8,speed_limit:f32,from_road:usize,to_road:usize,end_speed_limit:f32){
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
        road.obstacle_map.insert(OrderedFloat((road.length*10.0).round()/10.0),road.end_speed_limit);
        self.roads.push(road);
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
    pub from_road: usize,
    pub to_road: usize,
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
    pub destination_position: f32
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
    world.add_road((0.0,0.0,0.0),(0.0,0.0,500.0),1,100.0,0,1,10.0);
    world.add_road((0.0,0.0,500.0),(0.0,0.0,0.0),1,100.0,1,0,10.0);
    world.add_vehicle(0.0,0.0,5.0,-10.0,0,200.0,1,250.0);
    world.add_vehicle(0.0,0.0,4.0,-7.0,1,250.0,0,311.0);
    return world;
}
