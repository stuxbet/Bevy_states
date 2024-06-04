use bevy::prelude::*;
mod states;
mod events;

use states::StatePlugin;
use states::MachineState;
use events::SimpleEventPlugin;
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
        // // Add state handling
    .add_plugins(StatePlugin)
    //this is just a debug print statement to see the current state
    .add_systems(Update, state_in_now)
    //event stuff
    .add_plugins(SimpleEventPlugin)

    .run();
}
fn state_in_now(state: Res<State<MachineState>>){
    println!("state: {:?}", state)
}