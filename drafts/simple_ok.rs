//Base
use bevy::prelude::*;
use bevy_config_cam::*;
use bevy::gltf::Gltf;
use bevy::gltf::GltfMesh;

/// Helper resource for tracking our asset
struct MyAssetPack(Handle<Gltf>);

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ConfigCam)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 22.0, //12.0,          // default: 12.0
            ..Default::default()
        })
        .insert_resource(PlayerSettings {
            pos: Vec3::new(2., 0., 0.),
            player_asset: "C:/Rust/PlovdivGameJam2022/assets/models/drone.gltf",
            //"C:/Rust/bevy/assets/models/drone/drone.gltf#Scene0",// "models/craft_speederA.glb#Scene0",
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .run();
        
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,    
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cl: ResMut<CamLogic>,
    asset_server: Res<AssetServer>,   
    ////my: Res<MyAssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
    ) 
{
    // plane for the cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    
    commands.spawn_scene(asset_server.load("C:/Rust/bevy/assets/models/drone/drone.gltf#Scene0"));
        
    //Not only spawn, insert
    ////  let gltf = asset_server.load("C:/Rust/bevy/assets/models/drone/drone.gltf#Scene0");    
    
    ////// commands.insert_resource(MyAssetPack(gltf));//
    
    /*
    c:\Rust\bevy_config_cam>cargo run --example simple
   Compiling bevy_config_cam v0.1.3 (C:\Rust\bevy_config_cam)
    Finished dev [unoptimized + debuginfo] target(s) in 5.21s
     Running `target\debug\examples\simple.exe`
thread 'Compute Task Pool (1)' panicked at 'Requested resource does not exist: simple::MyAssetPack', C:\Users\toshb\.cargo\registry\src\github.com-1ecc6299db9ec823\bevy_ecs-0.5.0\src\system\system_param.rs:244:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'task has failed', C:\Users\toshb\.cargo\registry\src\github.com-1ecc6299db9ec823\async-task-4.0.3\src\task.rs:368:45
*/
    
    //commands.insert_resource(assets_gltf(gltf));    

    let prop: Handle<Mesh> = asset_server.get_handle("C:/Rust/bevy/assets/models/drone/drone.gltf#Scene0");
    
    /*
    if let Some(opt) = meshes.get_mut(&prop)
    {
        println!("Some?");
        let opt2 = opt.clone();        
        info!("{:?}", opt.primitive_topology());        
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(opt2), //Mesh::from(opt)), //.unwrap())),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        });        
    }
    
    if let Some(gltf) = assets_gltf.get(&my.0) {
        println!("Some(gltf!)");                
        let carwheel = assets_gltfmesh.get(&gltf.meshes[9]).unwrap();
        // Spawn a PBR entity with the mesh and material of the first GLTF Primitive
        commands.spawn_bundle(PbrBundle {
            mesh: carwheel.primitives[0].mesh.clone(),
            // (unwrap: material is optional, we assume this primitive has one)
            material: carwheel.primitives[0].material.clone().unwrap(),
            ..Default::default()
        });
      }       
      */     
    // cube, set as target
    cl.target = Some(
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..Default::default()
            })
            .id(),
    );

 // cube, set as target NO
//   cl.target = Some(
//         commands.spawn_scene(asset_server.load("C:/Rust/bevy/assets/models/drone/drone.gltf#Scene0").
//    );
    
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()});
}       
