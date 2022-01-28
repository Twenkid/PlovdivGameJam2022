use bevy::prelude::*;
use log::info;

// Should: Add a drone asset
// Should: Add a be able to manipulate the drone model
// Should(?): Be able to listen for MAVSDK messages (maybe)
pub struct DronePlugin;

impl Plugin for DronePlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here

        app
        .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin::default())
        .add_startup_system(setup)
        //     .add_system(greet_people);
        ;
    }
}


/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("Setup Drone asset");

    let _scenes: Vec<HandleUntyped> = asset_server.load_folder("models").unwrap();
    
    let drone_handle: Handle<Mesh> = asset_server.load("models/drone.gltf#RootNode (gltf orientation matrix)");

     // You can also add assets directly to their Assets<T> storage:
     let drone_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        ..Default::default()
    });

    // drone
    commands.spawn_bundle(PbrBundle {
        mesh: drone_handle,
        material: drone_material_handle.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        // TODO: Should we scale?
        // .with_scale(Vec3::new(2.0, 2.0, 2.0)),
        ..Default::default()
    });


    // plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .insert_bundle(bevy_mod_picking::PickableBundle::default())
        .insert(bevy_transform_gizmo::GizmoTransformable);
    // cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert_bundle(bevy_mod_picking::PickableBundle::default())
        .insert(bevy_transform_gizmo::GizmoTransformable);
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert_bundle(bevy_mod_picking::PickingCameraBundle::default())
        .insert(bevy_transform_gizmo::GizmoPickSource::default());
}
