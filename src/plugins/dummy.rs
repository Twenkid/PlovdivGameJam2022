use std::sync::{atomic::AtomicUsize, Arc};

use bevy::prelude::*;
use chrono::{DateTime, Utc};
use mav_sdk::grpc::telemetry::Quaternion;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Response<T> {
    timestamp: DateTime<Utc>,
    response: T,
}

#[derive(Debug, Component)]
struct LoadQuaternions {
    responses: Arc<Vec<Response<Quaternion>>>,
    next_response: AtomicUsize,
}


static QUATERNIONS_FILE: &str =
    include_str!("../../dummy/2021-10-24 20_51_18.910675477 UTC-quaternions.json");
static POSITIONS_FILE: &str =
    include_str!("../../dummy/2021-10-24 20_51_18.910675477 UTC-position.json");

/// Not used at the moment but could be
static _QUATERNIONS: Lazy<Vec<Response<Quaternion>>> = Lazy::new(|| {
    QUATERNIONS_FILE
        .lines()
        .map(|record| serde_json::from_str(record))
        .collect::<Result<_, _>>()
        .expect("Should deserialize all lines in file")
});

pub struct DummyPlugin;

impl Plugin for DummyPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here

        app.add_event::<Quaternion>()
            .add_startup_system(load_quaternions)
            .add_system(send_quaternion_events);
    }
}

/// Loads all quaternions and the first Quaternion in resources
/// 
fn load_quaternions(mut commands: Commands) {
    let quaternion_responses: Vec<Response<Quaternion>> = QUATERNIONS_FILE
        .lines()
        .map(|record| serde_json::from_str(record))
        .collect::<Result<_, _>>()
        .expect("Should deserialize all lines in file");

    // add initial quaternion and set index
    let initial_quaternion = quaternion_responses.get(0).expect("Should have at least 1 quaternion").clone();
    let load_quaternions = LoadQuaternions {
        responses: Arc::new(quaternion_responses),
        // start from index = 1 as 0 was the initial state of the drone
        next_response: AtomicUsize::from(1),
    };

    commands.insert_resource(initial_quaternion);
    commands.insert_resource(load_quaternions);
}

fn send_quaternion_events(time: Res<Time>) {

}
