use async_compat::CompatExt;
use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use futures_lite::future;
use mav_sdk::{
    grpc::{
        action::{ArmRequest, TakeoffRequest},
        manual_control::{SetManualControlInputRequest, StartAltitudeControlRequest},
        telemetry::{AttitudeQuaternionResponse, Position, PositionResponse, Quaternion},
    },
    Drone,
};
use tokio::sync::mpsc::{channel, Receiver, Sender};

/// Gazbeo running PX4 w/ video stream
const MAVSDK_SERVER: &str = "http://127.0.0.1:4000";

pub struct MavsdkPlugin;

impl Plugin for MavsdkPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here

        app
        .add_event::<Quaternion>()
            .add_event::<Position>()
            .add_startup_system(start_listeners)
            // .add_startup_system(start_offboard_control)
            .add_system(send_quaternion_events)
            .add_system(send_position_events)
            .add_system(moving_drone);
    }
}

fn send_quaternion_events(
    mut quaternion_events: EventWriter<Quaternion>,
    mut receiver: ResMut<Receiver<Quaternion>>,
) {
    // this does not work well as we might miss all the events in between!
    let quaternion = future::block_on(future::poll_once(receiver.recv().compat())).flatten();
    if let Some(quaternion) = quaternion {
        quaternion_events.send(quaternion);
        // info!("Sent event Quaternion");
    }
}

fn send_position_events(
    mut position_events: EventWriter<Position>,
    mut receiver: ResMut<Receiver<Position>>,
) {
    // this does not work well as we might miss all the events in between!
    let position = future::block_on(future::poll_once(receiver.recv().compat())).flatten();
    if let Some(position) = position {
        position_events.send(position);
    }
}

fn moving_drone(
    keyboard: Res<Input<KeyCode>>,
    drone: Res<Drone>,
    thread_pool: Res<AsyncComputeTaskPool>,
) {
    // Default to zeros
    //https://docs.rs/mav-sdk/0.1.0/mav_sdk/grpc/offboard/struct.AttitudeRate.html
    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.5;
    // yaw (rotation)
    let r = 0.0;

    // once a key is pressed, mutate the attitude rate
    for key in keyboard.get_pressed() {
        match key {
            // roll left (clock-wise) from front!
            KeyCode::A => {
                y += 0.5;
            }
            // roll right (clock-wise)
            KeyCode::D => {
                y -= 0.5;
            }
            // backwards (pitch)
            KeyCode::S => {
                x -= 0.5;
            }
            // forward (pitch)
            KeyCode::W => {
                x += 0.5;
            }
            // take-off
            KeyCode::T => {
                let mut drone = drone.clone();
                thread_pool
                    .spawn(async move {
                        let request = TakeoffRequest {};
                        match drone.action.takeoff(request).await {
                            Ok(response) => {
                                info!(
                                    "Got an Takeoff response: {:?}",
                                    response.get_ref().action_result
                                );
                            }
                            Err(err) => {
                                error!("Got an error status for Takeoff request: {:?}", err);
                            }
                        }
                    })
                    .detach();
            }
            // land
            KeyCode::L => {
                todo!("Drone land")
            }
            // Arm drone before take-off
            KeyCode::Z => {
                let mut drone = drone.clone();
                thread_pool
                    .spawn(async move {
                        let request = ArmRequest {};
                        match drone.action.arm(request).await {
                            Ok(response) => {
                                info!(
                                    "Got an Arm response: {:?}",
                                    response.get_ref().action_result
                                );
                            }
                            Err(err) => {
                                error!("Got an error status for Arm request: {:?}", err);
                            }
                        }
                    })
                    .detach();
            }
            // Attitude (trust)
            KeyCode::Space => {
                // MAX trust
                z = 1.0;
            }
            // skip the rest
            _ => {}
        }
    }

    let mut drone = drone.clone();

    thread_pool
        .spawn(async move {
            let request = SetManualControlInputRequest { x, y, z, r };
            match drone.manual_control.set_manual_control_input(request).await {
                Ok(_response) => {
                    // info!(
                    //     "Got an Manual response: {:?}",
                    //     response.get_ref().manual_control_result
                    // );
                }
                Err(err) => {
                    error!("Got an error status for request: {:?}", err);
                }
            }
        })
        .detach();
}

fn start_listeners(mut commands: Commands, thread_pool: Res<AsyncComputeTaskPool>) {
    // A channel for sending and receiving Quaternions from MAVSDK
    let (position_tx, position_rx) = channel::<Position>(250);
    let (quaternion_tx, quaternion_rx) = channel::<Quaternion>(250);

    // make a drone client!
    let drone =
        future::block_on(Drone::connect(MAVSDK_SERVER).compat()).expect("Should connect to drone");

    // add it as resource
    commands.insert_resource(drone.clone());

    let quat_drone = drone.clone();
    thread_pool
        .spawn(async move { quaternion_listener(quat_drone, quaternion_tx).compat().await })
        .detach();

    commands.insert_resource(quaternion_rx);


    let position_drone = drone.clone();
    thread_pool
        .spawn(async move { position_listener(position_drone, position_tx).compat().await })
        .detach();
    
    commands.insert_resource(position_rx);
    

    let mut start_manual_control_drone = drone.clone();
    match future::block_on(async move {
        let start_request = StartAltitudeControlRequest {};
        start_manual_control_drone
            .manual_control
            .start_altitude_control(start_request)
            .await
    }) {
        Ok(response) => info!(
            "Got a start manual control control response: {:?}",
            response.get_ref()
        ),
        Err(err) => error!(
            "Error status while starting manual control for drone: {}",
            err
        ),
    };
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
            tx.send(attitude_quaternion)
                .await
                .expect("Should not panic");
        }
    }
}

async fn position_listener(mut drone: Drone, tx: Sender<Position>) {
    let subscribe_position_request = mav_sdk::grpc::telemetry::SubscribePositionRequest {};
    let mut response = drone
        .telemetry
        .subscribe_position(subscribe_position_request)
        .await
        .expect("Should subscribe");

    while let Some(PositionResponse { position }) = response
        .get_mut()
        .message()
        .await
        .expect("Should get response")
    {
        if let Some(position) = position {
            tx.send(position).await.expect("Should not panic");
        }
    }
}
