use async_compat::CompatExt;
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use futures_lite::future;
use mav_sdk::{
    grpc::telemetry::{AttitudeQuaternionResponse, Quaternion},
    Drone,
};
use tokio::sync::mpsc::{Receiver, channel, Sender,};

/// Gazbeo running PX4 w/ video stream
const MAVSDK_SERVER: &str = "http://127.0.0.1:4000";

pub struct MavsdkPlugin;

impl Plugin for MavsdkPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here

        app
            .add_event::<Quaternion>()
            .add_startup_system(start_quaternion_listener)
            .add_system(send_quaternion_events)
        ;
    }
}

fn send_quaternion_events(
    mut quaternion_events: EventWriter<Quaternion>,
    mut receiver: ResMut<Receiver<Quaternion>>,
) {
    // make a drone client!

    // this does not work well as we might miss all the events in between!
    let quaternion = future::block_on(future::poll_once(receiver.recv().compat())).flatten();
    if let Some(quaternion) = quaternion {
        quaternion_events.send(quaternion);
        info!("Sent event Quaternion");
    }
}

fn start_quaternion_listener(
    mut commands: Commands,
    thread_pool: Res<AsyncComputeTaskPool>,
) {
    // A channel for sending and receiving Quaternions from MAVSDK
    let (tx, rx) = channel::<Quaternion>(250);

    // make a drone client!
    let drone = future::block_on(Drone::connect(MAVSDK_SERVER)
    .compat())
    .expect("Should connect to drone");

    // add it as resource
    commands.insert_resource(drone.clone());

    thread_pool.spawn(async move {
        let drone = drone.clone();

        quaternion_listener(drone, tx).compat().await
    }).detach();

    commands.insert_resource(rx);
}

async fn quaternion_listener(mut drone: Drone, tx: Sender<Quaternion>) {
    let subscribe_quaternion_request =
        mav_sdk::grpc::telemetry::SubscribeAttitudeQuaternionRequest {};
    let mut response = drone
        .telemetry
        .subscribe_attitude_quaternion(subscribe_quaternion_request)
        .await
        .expect("Should subscribe");

    while let Some(AttitudeQuaternionResponse {
        attitude_quaternion,
    }) = response
        .get_mut()
        .message()
        .await
        .expect("Should get response")
    {
        if let Some(attitude_quaternion) = attitude_quaternion {
            info!("Received {:?}", &attitude_quaternion);

            tx.send(attitude_quaternion)
                .await
                .expect("Should not panic");
        }
    }
}