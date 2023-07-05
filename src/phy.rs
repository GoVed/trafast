pub use crate::comp::*;
use ordered_float::OrderedFloat;

// 
pub fn update(t:f32,world:&mut World){
    let vehicles = &mut world.vehicles;
    let roads =  &world.roads;
    for mut vehicle in vehicles.iter_mut(){
        let (dist,end_speed_limit) = check_road_end(vehicle,roads);
        if check_destination_start_break(vehicle,roads){
            decrease_speed(&mut vehicle,&t,dist,0.0);
        }
        else{
            if dist != 0.0{
                decrease_speed(&mut vehicle,&t,dist,end_speed_limit);
            }
            else{
                increase_speed(&mut vehicle,roads,&t);
            }
        }
        
        
    }
    
}

fn check_road_end(vehicle:&Vehicle,roads:&Vec<Road>) -> (f32,f32){
    let mut nearest_obstacle:OrderedFloat<f32> = roads[vehicle.on_road].length+1.0;
    let vehicle_position:OrderedFloat<f32> = vehicle.position.into();
    
    for (key,_) in roads[vehicle.on_road].obstacle_map.iter(){
        if vehicle_position <= *key && vehicle_position >= *key - vehicle.watch_distance{
            if *key - vehicle.position < nearest_obstacle{
                nearest_obstacle = *key - vehicle.position;
            }
        }
    }

    if nearest_obstacle < roads[vehicle.on_road].length+1.0{
        (nearest_obstacle.into(),roads[vehicle.on_road].end_speed_limit)
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
    if vehicle.velocity > end_speed_limit{
        let v:f32 = &vehicle.velocity +  &vehicle.break_decceleration * t;
        if v < end_speed_limit{
            let early_stop_distance:f32 = vehicle.velocity;
            let mut required_decceleration:f32 = (&vehicle.velocity.powi(2) - end_speed_limit.powi(2))/(2.0*(dist+early_stop_distance));
            if required_decceleration > vehicle.break_decceleration{
                required_decceleration = vehicle.break_decceleration;
            }
            let targett:f32 = (&vehicle.velocity - end_speed_limit)/&required_decceleration;
            //Update position
            vehicle.position += &vehicle.velocity * targett + (&required_decceleration * targett.powi(2)/2.0);
            //Update velocity
            vehicle.velocity = end_speed_limit;            
            //Update position after reaching target
            vehicle.position += &vehicle.velocity * (t - targett);
        }
        else{
            //Update position
            vehicle.position += &vehicle.velocity * t + (&vehicle.break_decceleration * t.powi(2)/2.0);
            //Update velocity
            vehicle.velocity = v;
        }
    }
    else{
        vehicle.position += &vehicle.velocity * t;
    }
}


//Check if vehicle is near destination and start breaking
fn check_destination_start_break(vehicle:&Vehicle,roads:&Vec<Road>) -> bool{
    if vehicle.on_road == vehicle.destination{

        //Considered half break decceleration as normal decceleration
        let break_distance:f32 = vehicle.velocity.powi(2)/(vehicle.break_decceleration);
        if vehicle.destination_position - vehicle.position < break_distance{
            return true;
        }
    }
    false
}