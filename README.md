# Drone Rider
## @PlovdivGameJam2022 "Duality" theme

The Drone Rider is not just a game but User Interface for drones!

You can either use a drone simulation or...

You can actually use it to control a real-life drone!

**Additional Materials**

The project page on the Global Game Jam site: https://globalgamejam.org/2022/games/drone-simulator-8

A 5-minute video presentation, given by **elpiel** at the event. Currently only in Bulgarian, we may add a translated/dubbed version: https://youtu.be/rAX33LyyB9Q?t=4765

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

### Team: "Айляците": "Aylyatsite" or **"The Aylyaks" ~ "The Idles"**(see note)

Idea, research and lead developer, Linux: **@elpiel**

Second developer, testing, R&D, Windows: **@Twenkid**

### Assets

### Drone model as of 30.1.2022: (C) **pythondesign**, https://sketchfab.com/3d-models/petrone-battle-drone-149aa3e19502471db0dd501bd35bbbc9 

Do not distribute as a stand-alone file and please purchase it for other projects. See the standard license: https://sketchfab.com/licenses

### Notes

"Aylyak" is a jargon word for Plovdiv citizens, who are calm, free of worries, enjoy themselves etc.
See this BBC article from 2020 which provides an explanation:
https://www.bbc.com/travel/article/20201104-europes-city-of-dawdlers-and-loafers

_"The Bulgarian city of Plovdiv has an almost untranslatable word – “aylyak” – that manifests as a refusal to get caught up in the rat race and a scepticism about the value of overwork."_

We participated with the goal to test the capabilities of Bevyengine etc. for our goals with the project and wanted to study and "hack" this new engine for a very short time, without prior experience with it. We couldn't invest a full time focus during part of the event, either*. It's just a beginning and you may join and help as improve it!

* An additional technical hurdle was working on both Linux and Windows, which required different settings of Rust or reverting to more conservative ones due to the Windows environment.

### To do, as of 1.2.2022

* Fix the orientation of the drone - it needs a transform to rotate it to a neutral position, there's something messed up with the quaternions, which we couldn't fix at the event. Another option: replace the model with another one, having a less restrictive license.
* Fix the background to sky blue etc., add a greeny grass plane for the ground etc.
* Complete the "dummy" function which reads positions and rotations of the drone from a json file and allows running without invoking Gazebo from the Docker container.
