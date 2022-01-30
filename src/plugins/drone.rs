use bevy::{gltf::Gltf, prelude::*, scene::InstanceId};
use log::info;
use mav_sdk::grpc::telemetry::Quaternion;

pub struct DronePlugin;

// Resource to hold the scene `instance_id` until it is loaded
#[derive(Default)]
struct SceneInstance(Option<InstanceId>);

#[derive(Component)]
struct DroneComponent;

impl Plugin for DronePlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here

        app
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .init_resource::<SceneInstance>()
        // Load drone asset
        .add_startup_system(load_drone_asset)
        // setup rest of plugin
        .add_startup_system(setup)
        .add_system(scene_update)
        .add_system(move_scene_entities)
        // spawn the drone asset when ready
        // .add_system(spawn_drone)
        ;
    }
}

struct DroneAssetPack(Handle<Gltf>);

fn load_drone_asset(mut commands: Commands, asset_server: Res<AssetServer>) {
    let gltf = asset_server.load("models/drone.gltf");

    commands.insert_resource(DroneAssetPack(gltf));
}

// fn spawn_drone(
//     mut commands: Commands,
//     assets_gltf: Res<Assets<Gltf>>,
//     drone: Res<DroneAssetPack>,
//     mut quaternion_events: EventReader<Quaternion>,
// ) {
//      if let Some(drone) = assets_gltf.get(&drone.0) {
//         commands
//             .spawn_bundle((
//                 Transform::from_scale(Vec3::new(0.03, 0.03, 0.03)),
//                 GlobalTransform::identity(),
//             ))
//             .with_children(|parent| {
//                 parent.spawn_scene(drone.scenes[0].clone());
//             });
//     }
// }

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // assets_gltf: Res<Assets<Gltf>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut scene_instance: ResMut<SceneInstance>,
) {
    info!("Setup Drone asset");

    let drone: Handle<Gltf> = asset_server.load("models/drone.gltf");
    // let drone_scene: Handle<Scene> = asset_server.load("models/drone.gltf#Scene0");
    // insert drone model as resource
    commands.insert_resource(DroneAssetPack(drone.clone()));

    // let drone_scene_id = commands
    //     .spawn_bundle((
    //         Transform::from_scale(Vec3::new(0.03, 0.03, 0.03)),
    //         GlobalTransform::identity(),
    //     ))
    //     .with_children(|parent| {
    //         parent.spawn_scene(drone_scene);
    //     }).id();

    // commands.insert_resource(DroneInstance(drone_scene_id));
    let instance_id = scene_spawner.spawn(asset_server.load("models/drone.gltf#Scene0"));
    scene_instance.0 = Some(instance_id);

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
            // transform: Transform::from_xyz(10.0, 10.5, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            transform: Transform::from_xyz(220.0, 120.5, 170.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert_bundle(bevy_mod_picking::PickingCameraBundle::default())
        .insert(bevy_transform_gizmo::GizmoPickSource::default());
}

fn scene_update(
    mut commands: Commands,
    scene_spawner: Res<SceneSpawner>,
    scene_instance: Res<SceneInstance>,
    mut done: Local<bool>,
) {
    if !*done {
        if let Some(instance_id) = scene_instance.0 {
            if let Some(entity_iter) = scene_spawner.iter_instance_entities(instance_id) {
                entity_iter.for_each(|entity| {
                    commands.entity(entity).insert(DroneComponent);
                });
                *done = true;
            }
        }
    }
}

fn move_scene_entities(
    mut quaternion_events: EventReader<Quaternion>,
    mut scene_entities: Query<&mut Transform, With<DroneComponent>>,
) {
    for event in quaternion_events.iter() {
        for mut transform in scene_entities.iter_mut() {
            // transform.scale = Vec3::new(0.03, 0.03, 0.03);
            transform.rotation = Quat::from_xyzw(event.x, event.y, event.z, event.w);
        }
    }
}
