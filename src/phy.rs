pub use crate::comp::*;
use ordered_float::OrderedFloat;

//Update vehicle position and velocity
pub fn update_comp(t:f32,world:&mut World){
    let vehicles = &mut world.vehicles;
    let mut roads =  &mut world.roads;
    let mut remove_vehicles:Vec<usize> = Vec::new();

    let mut i = 0;
    for mut vehicle in vehicles.iter_mut(){
        //Update obstacle map
        //.x1 is for vehicles
        let run_behind:f32 = 1.5;
        roads[vehicle.on_road].obstacle_map.remove(&OrderedFloat(((vehicle.position*10.0).round()/10.0)-0.01-run_behind));
        let (dist,end_speed_limit) = check_road_obstacle(vehicle,roads);
        if check_destination_start_break(vehicle){
            let dist:f32 = vehicle.destination_position - vehicle.position;
            decrease_speed(&mut vehicle,&t,dist,0.0);
        }
        else{
            if dist != 0.0 && vehicle.velocity > end_speed_limit{
                decrease_speed(&mut vehicle,&t,dist,end_speed_limit);
            }
            else{
                increase_speed(&mut vehicle,roads,&t);
            }
        }

        //Check if vehicle has reached destination
        if vehicle.on_road == vehicle.destination && vehicle.position >= vehicle.destination_position-10.0 && vehicle.velocity == 0.0{
            remove_vehicles.push(i);
        }
        else{
            roads[vehicle.on_road].obstacle_map.insert(OrderedFloat(((vehicle.position*10.0).round()/10.0)-0.01-run_behind),vehicle.velocity);
        }
        
        i+=1;
    }
    
    //Remove vehicles that have reached destination
    remove_vehicles.sort();
    for i in remove_vehicles.iter(){
        vehicles.remove(*i);
    }
}

fn check_road_obstacle(vehicle:&mut Vehicle,roads:&Vec<Road>) -> (f32,f32){
    let mut nearest_obstacle:OrderedFloat<f32> = roads[vehicle.on_road].length+1.0;
    let vehicle_position:OrderedFloat<f32> = vehicle.position.into();
    let mut nearest_obstacle_speed:f32 = 0.0;
    if vehicle_position >= roads[vehicle.on_road].length.into(){
        vehicle.on_road = vehicle.path.remove(0);
        vehicle.position = 0.0;
    }
    for (key,_) in roads[vehicle.on_road].obstacle_map.iter(){
        if vehicle_position <= *key && vehicle_position >= *key - vehicle.watch_distance{
            if *key - vehicle.position < nearest_obstacle{
                nearest_obstacle = *key - vehicle.position;
                nearest_obstacle_speed = *roads[vehicle.on_road].obstacle_map.get(key).unwrap();
            }
        }
    }

    if nearest_obstacle < roads[vehicle.on_road].length+1.0{
        (nearest_obstacle.into(),nearest_obstacle_speed)
    }
    else{
        return (0.0,0.0);
    }
}

fn increase_speed(vehicle:&mut Vehicle,roads:&Vec<Road>,t:&f32){
    if vehicle.velocity < roads[vehicle.on_road].speed_limit{
        let v:f32 = &vehicle.velocity +  &vehicle.acceleration * t;
        if v > roads[vehicle.on_road].speed_limit{
            let targett:f32 = (roads[vehicle.on_road].speed_limit - &vehicle.velocity)/&vehicle.acceleration;
            //Update position
            vehicle.position += &vehicle.velocity * targett + (&vehicle.acceleration * targett.powi(2)/2.0);
            //Update velocity
            vehicle.velocity = roads[vehicle.on_road].speed_limit;            
            //Update position after reaching target
            vehicle.position += &vehicle.velocity * (t - targett);
        }
        else{
            //Update position
            vehicle.position += &vehicle.velocity * t + (&vehicle.acceleration * t.powi(2)/2.0);
            //Update velocity
            vehicle.velocity = v;
        }
    }
    else{
        vehicle.position += &vehicle.velocity * t;
    }
    
}


//Decrease speed of vehicle
fn decrease_speed(vehicle:&mut Vehicle,t:&f32,dist:f32,end_speed_limit:f32){
    let normal_end_speed_limit:f32 = end_speed_limit*0.75;
    if vehicle.velocity > normal_end_speed_limit{
        //Leeway for early stop, keeping distance when in high velocity
        let mut early_stop_distance:f32 = dist - (2.0 * vehicle.velocity);
        if early_stop_distance < 0.0{
            early_stop_distance = 0.0;
        }
        //Calculate required decceleration and check if it is greater than break decceleration a= (v^2 - u^2)/2s
        let mut required_decceleration:f32 = (vehicle.velocity.powi(2) - normal_end_speed_limit.powi(2))/(2.0*early_stop_distance);
        if required_decceleration > -1.0*vehicle.break_decceleration || required_decceleration < 0.0{            
            required_decceleration = vehicle.break_decceleration;
        }        
        let v:f32 = &vehicle.velocity +  required_decceleration * t;
        if v < normal_end_speed_limit{
            
            //Calculate time to reach target
            let targett:f32 = (&vehicle.velocity - normal_end_speed_limit)/&required_decceleration;
            //Update position
            vehicle.position += &vehicle.velocity * targett + (&required_decceleration * targett.powi(2)/2.0);
            //Update velocity
            vehicle.velocity = normal_end_speed_limit;            
            //Update position after reaching target
            vehicle.position += &vehicle.velocity * (t - targett);
        }
        else{
            //Update position
            vehicle.position += &vehicle.velocity * t + (required_decceleration * t.powi(2)/2.0);
            //Update velocity
            vehicle.velocity = v;
        }
    }
    else{
        vehicle.position += &vehicle.velocity * t;
    }
}


//Check if vehicle is near destination and start breaking
fn check_destination_start_break(vehicle:&Vehicle) -> bool{
    if vehicle.on_road == vehicle.destination{

        //Considered half break decceleration as normal decceleration
        let break_distance:f32 = -1.0*vehicle.velocity.powi(2)/(vehicle.break_decceleration);
        
        if vehicle.destination_position - vehicle.position < break_distance{
            return true;
        }
    }
    false
}

