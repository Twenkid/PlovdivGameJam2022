//! # Drone UI

pub mod plugins {
    pub use drone::DronePlugin;
    pub use mavsdk::MavsdkPlugin;

    mod drone;
    mod mavsdk;
}