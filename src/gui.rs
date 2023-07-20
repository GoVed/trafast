use bevy::prelude::*;

pub use crate::comp::*;
pub use crate::comp::World;
pub use crate::phy::*;


pub fn run(){
    App::new()
    .insert_resource(World {
        roads: Vec::new(),
        vehicles: Vec::new(),
    })
    .add_startup_system(create_sample_world)
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_directional_light)
    .add_startup_system(set_initial_state)
    .add_system(update_frame)
    .add_plugins(DefaultPlugins)
    .run(); 
}




fn create_sample_world(mut world: ResMut<World>){
    world.add_road((0.0,10.0,0.0),(500.0,10.0,0.0),1,100.0,0,1,10.0);
    world.add_road((500.0,-10.0,0.0),(0.0,-10.0,0.0),1,100.0,1,0,10.0);
    world.add_vehicle(0.0,0.0,5.0,-10.0,0,200.0,1,250.0);
    world.add_vehicle(0.0,0.0,4.0,-7.0,1,250.0,0,250.0);
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;


#[derive(Component)]
struct BevyVehicle;

fn update_frame(mut world: ResMut<World>, time: Res<Time>, mut query: Query<(&mut Transform,Entity), With<BevyVehicle>>, mut commands: Commands, mut text_query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut text_query {
        text.sections[1].value = format!("{:.2}", 1.0/time.delta_seconds());
    }
    if !query.is_empty() {  
        update_comp(time.delta_seconds(),&mut world);     
        
        let mut i = 0;
        for (mut t,v) in query.iter_mut() {
            let veh = &world.vehicles[i];
            if i >= world.vehicles.len() {
                commands.entity(v).despawn();
            }
            else{
                
                let road_from = Vec3::new(world.roads[veh.on_road].from.0, world.roads[veh.on_road].from.1, world.roads[veh.on_road].from.2);
                let road_to = Vec3::new(world.roads[veh.on_road].to.0, world.roads[veh.on_road].to.1, world.roads[veh.on_road].to.2);
                let road_length = (road_to - road_from).length();
                let position = road_from + (road_to - road_from) * (veh.position/road_length);
                t.translation = position;
            }
            println!("V{}\t{}\t{}",i,veh.velocity,veh.position);
            i+=1;
        }
    }
    else{
        update_comp(0.0,&mut world);
    }
    
    
}

fn set_initial_state(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>,mut world: ResMut<World>, asset_server: Res<AssetServer>){
    
    for road in &world.roads {
        let from_vec3 = Vec3::new(road.from.0, road.from.1, road.from.2);
        let to_vec3 = Vec3::new(road.to.0, road.to.1, road.to.2);
        let center = (from_vec3 + to_vec3) / 2.0;
        let size = (from_vec3 - to_vec3).length();
        let width = 10.0;
        let rotation = Quat::from_rotation_z((to_vec3 - from_vec3).y.atan2((to_vec3 - from_vec3).x));
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad { size: Vec2::new(size,width), flip: false})),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform{
                translation: center,
                rotation: rotation,
                ..Default::default()
            },
            ..Default::default()
        });
    }

    // Spawn the vehicles on the road
    for vehicle in &world.vehicles {
        let vehicle_size = 5.0; // Adjust the size of the vehicle
        let road_from = Vec3::new(world.roads[vehicle.on_road].from.0, world.roads[vehicle.on_road].from.1, world.roads[vehicle.on_road].from.2);
        let road_to = Vec3::new(world.roads[vehicle.on_road].to.0, world.roads[vehicle.on_road].to.1, world.roads[vehicle.on_road].to.2);
        let road_length = (road_to - road_from).length();
        let position = road_from + (road_to - road_from) * (vehicle.position/road_length);
        
        commands.spawn((PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: vehicle_size })),
            material: materials.add(Color::rgb(0.8, 0.2, 0.2).into()),
            transform: Transform {
                translation: position,
                ..Default::default()
            },
            ..Default::default()
        },BevyVehicle));
    }

    // Text with multiple sections
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 20.0,
                color: Color::GOLD,
            }),
        ]),
        FpsText,
    ));
}

#[derive(Component)]
struct WorldCamera;
fn spawn_camera(mut commands: Commands){
    commands.spawn((Camera3dBundle{
        transform: Transform{
            translation: Vec3::new(100.0, -250.0, 500.0),
            rotation: Quat::from_rotation_x(0.3),
            ..Default::default()
        },
        ..Default::default()
    },
    WorldCamera));
}



fn spawn_directional_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            ..default()
        }
        .into(),
        ..default()
    });
}