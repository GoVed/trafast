use bevy::prelude::*;

pub use crate::comp::*;
pub use crate::comp::World;
pub use crate::phy::*;
use std::fs::File;
use std::io::Read;


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
    .add_system(file_drag_and_drop_system)
    .run(); 
}

fn file_drag_and_drop_system(mut events: EventReader<FileDragAndDrop>,mut world: ResMut<World>) {
    for event in events.iter() {
        if let FileDragAndDrop::DroppedFile { window, path_buf } = event {
            println!("Dropped file with path: {:?}, in window id: {:?}", path_buf, window);
            let mut file = File::open(path_buf).expect("Unable to open");
            let mut contents = String::new();
            file.read_to_string(&mut contents);
            world.load_json(contents);
        }
    }
}



//Creates a sample world with 2 roads and 2 vehicles
fn create_sample_world(mut world: ResMut<World>){
    world.add_road((0.0,10.0,0.0),(500.0,10.0,0.0),1,100.0,0,1,5.0);
    world.add_road((500.0,-10.0,0.0),(0.0,-10.0,0.0),1,100.0,1,0,7.5);
    world.add_vehicle(0.0,0.0,5.0,-10.0,0,200.0,1,250.0);
    world.add_vehicle(50.0,0.0,3.0,-20.0,0,150.0,1,100.0);
    world.add_vehicle(0.0,0.0,4.0,-7.0,1,250.0,0,250.0);
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

//A struct to help identify the vehicle
#[derive(Component)]
struct BevyVehicle;

//A struct to help identify the road
#[derive(Component)]
struct BevyRoad;


//Updates the frame
fn update_frame(
    mut world: ResMut<World>, 
    time: Res<Time>, 
    mut param_set: ParamSet<'_,'_, (
        Query<(&mut Transform, Entity), With<BevyVehicle>>,
        Query<(&mut Transform, Entity), With<BevyRoad>>
    )>, 
    mut commands: Commands, 
    mut text_query: Query<&mut Text, With<FpsText>>, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>) {
    
    // Update the FPS counter text
    for mut text in &mut text_query {
        text.sections[1].value = format!("{:.2}", 1.0/time.delta_seconds());
    }

    
    //Update the vehicle position
    update_comp(time.delta_seconds(),&mut world);     

    
        
    let mut i = 0;

    //Update the vehicle position in the GUI
    for (mut t,v) in param_set.p0().iter_mut() {
        

        //Despawn the vehicle in GUI if it is not in the world
        if i >= world.vehicles.len() {
            commands.entity(v).despawn();
        }
        else{
            let veh = &world.vehicles[i];
            //Calculate the position of the vehicle
            let road_from = Vec3::new(world.roads[veh.on_road].from.0, world.roads[veh.on_road].from.1, world.roads[veh.on_road].from.2);
            let road_to = Vec3::new(world.roads[veh.on_road].to.0, world.roads[veh.on_road].to.1, world.roads[veh.on_road].to.2);
            let road_length = (road_to - road_from).length();
            let position = road_from + (road_to - road_from) * (veh.position/road_length);
            t.translation = position;
        }

        i+=1;
    }

    //Spawn the new vehicles in GUI
    if param_set.p0().iter_mut().len() < world.vehicles.len(){
        for i in param_set.p0().iter_mut().len()..world.vehicles.len(){
            spawn_vehicle(&mut commands, &mut meshes, &mut materials, &world.vehicles[i], &world);
        }
    }

    i = 0;
    //Update the road position in the GUI
    for (mut t,v) in param_set.p1().iter_mut() {
        //Despawn the road in GUI if it is not in the world
        if i >= world.roads.len() {
            commands.entity(v).despawn();
        }
        else{
            let road = &world.roads[i];
            //Calculate the position of the road
            let from_vec3 = Vec3::new(road.from.0, road.from.1, road.from.2);
            let to_vec3 = Vec3::new(road.to.0, road.to.1, road.to.2);
            let center = (from_vec3 + to_vec3) / 2.0;
            let size = (from_vec3 - to_vec3).length();
            let width = 10.0;
            let rotation = Quat::from_rotation_z((to_vec3 - from_vec3).y.atan2((to_vec3 - from_vec3).x));
            t.translation = center;
            t.rotation = rotation;
        }
        i+=1;
    }

    //Spawn the new roads in GUI
    if param_set.p1().iter_mut().len() < world.roads.len(){
        for i in param_set.p1().iter_mut().len()..world.roads.len(){
            spawn_road(&mut commands, &mut meshes, &mut materials, &world.roads[i]);
        }
    }
    
    
}

fn spawn_road( commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>,road: &Road){
    //Calculate the position of the road
    let from_vec3 = Vec3::new(road.from.0, road.from.1, road.from.2);
    let to_vec3 = Vec3::new(road.to.0, road.to.1, road.to.2);
    let center = (from_vec3 + to_vec3) / 2.0;
    let size = (from_vec3 - to_vec3).length();
    let width = 10.0;
    let rotation = Quat::from_rotation_z((to_vec3 - from_vec3).y.atan2((to_vec3 - from_vec3).x));

    //Spawn the road
    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Quad { size: Vec2::new(size,width), flip: false})),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform{
            translation: center,
            rotation: rotation,
            ..Default::default()
        },
        ..Default::default()
    },BevyRoad));
}

fn spawn_vehicle(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>,vehicle: &Vehicle, world: &ResMut<World>){
    //Calculate the position of the vehicle
    let road_from = Vec3::new(world.roads[vehicle.on_road].from.0, world.roads[vehicle.on_road].from.1, world.roads[vehicle.on_road].from.2);
    let road_to = Vec3::new(world.roads[vehicle.on_road].to.0, world.roads[vehicle.on_road].to.1, world.roads[vehicle.on_road].to.2);
    let road_length = (road_to - road_from).length();
    let position = road_from + (road_to - road_from) * (vehicle.position/road_length);

    //Spawn the vehicle
    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 5.0 })),
        material: materials.add(Color::rgb(0.8, 0.2, 0.2).into()),
        transform: Transform {
            translation: position,
            ..Default::default()
        },
        ..Default::default()
    },BevyVehicle));
}

//Sets the initial state of the GUI
fn set_initial_state(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>,mut world: ResMut<World>, asset_server: Res<AssetServer>){
    

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


//Spawn the camera
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


//Spawn the directional light
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