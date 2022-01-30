# Drone Rider
## @PlovdivGameJam2022 "Duality" theme

The Drone Rider is not just a game but User Interface for drones!
You can either use a drone simulation or...
You can actually use it to control a real-life drone!

### Works on: Windows & Linux


### How to run:

Requirements:
- [The Rust programming language](https://rust-lang.org)
- [Bevy v0.6 game engine requirements for Rust and prescuits](https://bevyengine.org/learn/book/getting-started/setup/)
- Docker with Docker-compose (for version of docker-compose files 3.8)


1. `docker-compose up -d`
It takes a few minutes (10-15 min) for the full simulation to be up and running, which includes:

- Drome simulation using Gazebo simulation using the PX4 autopilot.
- MAVSDK - a fancy gRPC server (written in C)

2. `cargo run`
    For rust setup with bevy see: https://bevyengine.org/learn/book/getting-started/setup/

3. (Optional) Use QGroundControl to connect to the drone in the simulation

Download and install QGroundControl: http://qgroundcontrol.com

