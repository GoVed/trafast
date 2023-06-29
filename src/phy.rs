pub use crate::comp::*;

pub fn update(t:f32,world:&mut World){
    let vehicles = &mut world.vehicles;
    let roads =  &world.roads;
    for mut vehicle in vehicles.iter_mut(){
        if check_road_end(vehicle,roads){

        }
        else{
            increase_speed(&mut vehicle,roads,&t);
        }
        
    }
    
}

fn check_road_end(vehicle:&Vehicle,roads:&Vec<Road>) -> bool{
    if vehicle.position >= *roads[vehicle.on_road].length{
        true
    }
    else{
        false
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