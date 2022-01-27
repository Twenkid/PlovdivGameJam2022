use bevy::prelude::*;

// Should: Add a drone asset
// Should: Add a be able to manipulate the drone model
// Should(?): Be able to listen for MAVSDK messages (maybe)
pub struct DronePlugin;

impl Plugin for DronePlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here

        // the reason we call from_seconds with the true flag is to make the timer repeat itself
        

        // app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
        //     .add_startup_system(add_people)
        //     .add_system(greet_people);
    }
}
