use bevy::prelude::*;
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};

use bevy_mod_picking::*;
use bevy_transform_gizmo::*;
//use bevy_mod_picking::PickableBundle;		
//use bevy_mod_picking::DefaultPickingPlugins;

//use bevy_mod_picking::{
    //DebugCursorPickingPlugin, DebugEventsPickingPlugin, DefaultPickingPlugins, PickableBundle,
//    PickingCameraBundle,
//};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(LookTransformPlugin)
        .add_plugin(OrbitCameraPlugin::default())
		.add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin::new(
            Quat::from_rotation_y(-0.2))// Align the gizmo to a different coordinate system. // Use TransformGizmoPlugin::default() to align to the scene's coordinate system.
        ) 
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
	asset_server: Res<AssetServer>	
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });
	
     commands.spawn_scene(asset_server.load("C:/Rust/bevy/assets/models/drone/drone.gltf#Scene0"));
	 
	 let mesh_handle: Handle<Mesh> = asset_server.load("C:/Rust/bevy/assets/models/drone/drone.gltf#Mesh9");
	 
	 /*
	 commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
            material: materials.add(Color::rgb(0.2, 0.7, 0.6).into()),
            transform: Transform::from_xyz(1.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert_bundle(PickableBundle::default()); // <- Makes the mesh pickable.
                                                   // light
*/

	 commands.spawn_bundle(PbrBundle {
            mesh: mesh_handle,
            material: materials.add(Color::rgb(0.9, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(bevy_transform_gizmo::GizmoTransformable);

		commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert_bundle(PickableBundle::default()); // <- Makes the mesh pickable.
                                                   // light

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 400.0, 4.0), //.. 8.0, 4.0
        ..Default::default()
    });

    commands.spawn_bundle(OrbitCameraBundle::new(
        OrbitCameraController::default(),
        PerspectiveCameraBundle::default(),
        Vec3::new(-2.0, 5.0, 5.0),
        Vec3::new(0., 0., 0.),
    ));
}
