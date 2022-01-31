# Drone Rider
## @PlovdivGameJam2022 "Duality" theme

The Drone Rider is not just a game but User Interface for drones!

You can either use a drone simulation or...

You can actually use it to control a real-life drone!

### Works on: Windows & Linux


### How to run

Requirements:
- [The Rust programming language](https://rust-lang.org)
- [Bevy v0.6 game engine requirements including for Rust (stable)](https://bevyengine.org/learn/book/getting-started/setup/)
- Docker with Docker-compose (for version of docker-compose files 3.8)


1. `docker-compose up -d`
It takes a few minutes (10-15 min) for the full simulation to be up and running, which includes:

- Drone simulation using Gazebo simulation and the PX4 autopilot.
- MAVSDK - a fancy gRPC server (written in C)
- Windows: `Docker Desktop` must be running before invoking `docker-compose`. Note that `Docker` requires `Windows Pro` license, because it relies on `HyperV` for the virtualization.

2. `cargo run`

Z - Arm the drone before take off, i.e. it's safe for take off.

T - Take off

W - Forward (pitch nose down)
A - Left (roll left)
S - Backward (pitch nose up)
D - Right (roll right)



3. (Optional) Use `QGroundControl` to connect to the drone in the simulation.

Download and run `QGroundControl`: http://qgroundcontrol.com

This will allow you to see the video feed from the drone alongside a map, telemetry (like altitude), GPS location, etc.

You can also return the drone to the take off location, make it land, take off and much more.

## Credits

**Team**

Idea, research and lead developer, Linux: **@elpiel**

Second developer, testing, R&D, Windows: **@Twenkid**

**Assets** 

**Drone model** as of 30.1.2022: (C) **pythondesign**, https://sketchfab.com/3d-models/petrone-battle-drone-149aa3e19502471db0dd501bd35bbbc9 

Do not distribute as a stand-alone file and please purchase it for other projects. See the standard license: https://sketchfab.com/licenses

