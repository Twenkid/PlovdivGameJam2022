use bevy::prelude::*;
use drone_ui::plugins::{DronePlugin, MavsdkPlugin};

fn main() {
        App::new()
            .insert_resource(WindowDescriptor {
                vsync: false, // Disabled for this demo to remove vsync as a source of input latency
                ..Default::default()
            })
            .insert_resource(Msaa { samples: 4 })
            // .insert_resource(rx)
            .add_plugins(DefaultPlugins)
            .add_plugin(DronePlugin)
            .add_plugin(MavsdkPlugin)
            .run();
}


