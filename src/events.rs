use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;

use crate::states::MachineState;

// Define a simple event
#[derive(Event)]
struct SimpleEvent {
    pub message: String,
    pub event_type: EventTypes
}
#[derive(Debug)]
enum EventTypes {
    ShitHitsTheFan,
    BigFire,
    Explosion,
    MiscComputerIssue
}

//System to send SimpleEvent when a key is pressed
fn send_event_system(
    mut event_writer: EventWriter<SimpleEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,

) {
    if keyboard_input.just_pressed(KeyCode::KeyA)  {
        event_writer.send(SimpleEvent {
            message: "Overtemp Detected event sent".to_string(),
            event_type: EventTypes::BigFire,
        });
    }
    if keyboard_input.just_pressed(KeyCode::KeyS)  {
        event_writer.send(SimpleEvent {
            message: "total meltdown Detected event sent".to_string(),
            event_type: EventTypes::ShitHitsTheFan,
        });
    }
    if keyboard_input.just_pressed(KeyCode::KeyD)  {
        event_writer.send(SimpleEvent {
            message: "large vibration and temp detected event sent".to_string(),
            event_type: EventTypes::Explosion,
        });
    }
    if keyboard_input.just_pressed(KeyCode::KeyW)  {
        event_writer.send(SimpleEvent {
            message: "Irratic sensor data detected event sent".to_string(),
            event_type: EventTypes::MiscComputerIssue,
        });
    }
}


// System to handle SimpleEvents and change state accordingly
fn handle_event_system(
    mut next_state: ResMut<NextState<MachineState>>,
    mut event_reader: EventReader<SimpleEvent>,
    mut state: Res<State<MachineState>>
) {
    for event in event_reader.read() {

        match event.event_type  {
            EventTypes::BigFire => {
                next_state.set(MachineState::Running);
            }
            EventTypes::Explosion => {
                next_state.set(MachineState::Idle);
            }
            EventTypes::ShitHitsTheFan => {
                next_state.set(MachineState::Running);
            }
            EventTypes::MiscComputerIssue => {
                next_state.set(MachineState::Idle);
            }
        }

        info!("{}:{:?}",event.message, event.event_type);
        info!("state{:?}",state);
    }
}

pub struct SimpleEventPlugin;

impl Plugin for SimpleEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SimpleEvent>()
            .add_systems(Update,send_event_system)
            .add_systems(Update,handle_event_system);
    }
}


