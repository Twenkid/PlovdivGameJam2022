use bevy::prelude::*;
use drone_ui::plugins::{DronePlugin, MavsdkPlugin};
use bevy::input::{keyboard::KeyCode, Input};

//use plugins::drone::DroneComponent;

#[derive(Component)]
struct DroneComponent;

//State
#[derive(Default)]
struct Game {
    player: Option<Entity>,
    w: f32,
    x: f32,
    y: f32,
    z: f32,    
}

fn main() {
        App::new()
            .init_resource::<Game>() //State must be initialized
            .insert_resource(WindowDescriptor {
                vsync: false, // Disabled for this demo to remove vsync as a source of input latency
                ..Default::default()
            })
            .insert_resource(Msaa { samples: 4 })
            // .insert_resource(rx)            
            .add_plugins(DefaultPlugins)
            .add_plugin(DronePlugin)
            .add_plugin(MavsdkPlugin)
            //.add_system(setup)
            .add_system(keyboard_input_system)  //keyboard system          
            .run();
}

/*
fn setup(mut game: ResMut<Game>){
    game.w = 0.3;
    game.x = 0.3;
    game.y = 0.3;
    game.z = 0.3;    
}*/

//mut scene_entities: Query<&mut Transform, With<DroneComponent>>
/// This system prints 'A' key state
fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>,
mut scene_entities: Query<&mut Transform, With<DroneComponent>>,
mut game: ResMut<Game>){
    
    let mut w: f32 = 0.71825;
    let mut x: f32 = -0.03567563;
    let mut y: f32 = 0.03567563;
    let mut z: f32 = 0.03; //0.69 ...
    //Add Time etc.  w.cos ... sin ...    
    println!("move_scene...");
    //if not events:
    let st: f32 = 0.1;
    
    //info!("keyboard_input_system");
    if keyboard_input.pressed(KeyCode::W) {
        info!("'W' currently pressed {}", game.y);         
           //w = w / 2.0;
           game.y+=st;
    }

    if keyboard_input.just_pressed(KeyCode::S) {
        info!("'S' just pressed {}", game.y);
        game.y-=st;
    }

    if keyboard_input.just_released(KeyCode::A) {
        info!("'A' just released {}", game.x);
        game.x+=st;
    }
    if keyboard_input.just_released(KeyCode::D) {
        info!("'D' just released {}", game.x);
        game.x-=st;
    }
            
     for mut transform in scene_entities.iter_mut() {
         info!("transform in scene_entities?");
            // transform.scale = Vec3::new(0.03, 0.03, 0.03);
            //transform.rotation = Quat::from_xyzw(event.x, event.y, event.z, event.w);
            transform.rotation = Quat::from_xyzw(game.x, game.y, game.z, game.w);            
        }
        info!("{},{},{}.{}", game.w, game.x, game.y, game.z);
}
