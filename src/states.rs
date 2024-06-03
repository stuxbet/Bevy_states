use bevy::prelude::*;
// use bevy_window::WindowPlugin;
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

/* 
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MachineState>().add_systems(
            Update,
            (
                game_state_input_events,
                //transition_to_in_game.run_if(in_state(GameState::GameOver)),
            ),
        );
    }
}
*/

impl Plugin for StatePlugin {
    fn build(&self,app: &mut App) {
        app.init_state::<MachineState>()
            .add_systems(Update,game_state_input_events);
    }
}


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

