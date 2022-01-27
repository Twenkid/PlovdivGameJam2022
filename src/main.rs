use bevy::prelude::*;
use drone_ui::plugins::DronePlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            vsync: false, // Disabled for this demo to remove vsync as a source of input latency
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DronePlugin)
        
        // .add_startup_system(add_drone)
        // .add_plugin(LookTransformPlugin)
        // .add_plugin(OrbitCameraPlugin::default())
        // .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        // .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin::new(
        //     Quat::from_rotation_y(-0.2))// Align the gizmo to a different coordinate system. // Use TransformGizmoPlugin::default() to align to the scene's coordinate system.
        // )
        // .add_system(hello_world)
        // .add_system(greet_people)
        .run();
}
