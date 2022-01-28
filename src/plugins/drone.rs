use bevy::{
    gltf::Gltf,
    prelude::*,
};
use log::info;

// Should: Add a drone asset
// Should: Add a be able to manipulate the drone model
// Should(?): Be able to listen for MAVSDK messages (maybe)
pub struct DronePlugin;

impl Plugin for DronePlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here

        app
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin::default())
        // .add_plugin(ConfigCam)
        // Load drone asset
        .add_startup_system(load_drone_asset)
        // setup rest of plugin
        .add_startup_system(setup)
        // spawn the drone asset when ready
            .add_system(spawn_drone);
    }
}

struct DroneAssetPack(Handle<Gltf>);

fn load_drone_asset(mut commands: Commands, asset_server: Res<AssetServer>) {
    let gltf = asset_server.load("models/drone.gltf");

    commands.insert_resource(DroneAssetPack(gltf));
}

fn spawn_drone(
    mut commands: Commands,
    assets_gltf: Res<Assets<Gltf>>,
    drone: Res<DroneAssetPack>,
) {
     if let Some(drone) = assets_gltf.get(&drone.0) {
        commands
            .spawn_bundle((
                Transform::from_scale(Vec3::new(0.03, 0.03, 0.03)),
                GlobalTransform::identity(),
            ))
            .with_children(|parent| {
                parent.spawn_scene(drone.scenes[0].clone());
            });
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

    let drone: Handle<Gltf> = asset_server.load("models/drone.gltf");
    // insert drone model as resource
    commands.insert_resource(DroneAssetPack(drone.clone()));

    // add light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(100.0, 100.0, 100.0),
        point_light: PointLight {
            color: Color::WHITE,
            /// Luminous power in lumens
            intensity: 800.0, // Roughly a 60W non-halogen incandescent bulb
            range: 3000.0,
            ..Default::default()
        },
        ..Default::default()
    });

    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(10.0, 10.5, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert_bundle(bevy_mod_picking::PickingCameraBundle::default())
        .insert(bevy_transform_gizmo::GizmoPickSource::default());

    // simple scene
    {
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
    }
}
