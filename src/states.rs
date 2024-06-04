use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;

#[derive(Debug,Clone,Copy,Default,States,Hash, Eq, PartialEq, Resource )]
pub enum MachineState {
    #[default]
    Idle,
    Running,
    Paused,
    EmergencyShutdown,
    EmergencyIdle,
    Shutdown,
}

pub struct StatePlugin;


//plugin system setup to add to app
impl Plugin for StatePlugin {
    fn build(&self,app: &mut App) {
        app.init_state::<MachineState>()
            .add_systems(Update,game_state_input_events);
    }
}

//just a test funciton to flip flop state when space is pressed
fn game_state_input_events(
    mut next_state: ResMut<NextState<MachineState>>,
    state: Res<State<MachineState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        match state.get() {
            MachineState::Idle => next_state.set(MachineState::Running),
            MachineState::Running => next_state.set(MachineState::Idle),
            _ => (),
        }
    }
}

