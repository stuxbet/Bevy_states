use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;

//FIXME:  this makes this system not fully indepenant from the states.rs so I think this should be looked at
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
    Emergency,
    PauseButtonHit
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
    if keyboard_input.just_pressed(KeyCode::KeyE)  {
        event_writer.send(SimpleEvent {
            message: "Emergency condition found sounding the alarm".to_string(),
            event_type: EventTypes::Emergency,
        });
    }
    if keyboard_input.just_pressed(KeyCode::KeyP)  {
        event_writer.send(SimpleEvent {
            message: "Irratic sensor data detected event sent".to_string(),
            event_type: EventTypes::PauseButtonHit,
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
        /*
        this match statement is where you would impement either single events triggering 
        statechanges may be able to add in parameters that take sensor data to chack for emergency conditions
        */


        match event.event_type  {
            EventTypes::BigFire => {
                next_state.set(MachineState::Running);
            }
            EventTypes::Emergency => {            
                next_state.set(MachineState::EmergencyShutdown);
            }
            EventTypes::ShitHitsTheFan => {
                next_state.set(MachineState::Running);
            }
            EventTypes::PauseButtonHit => {
                match state.get() {
                    MachineState::Paused => next_state.set(MachineState::Running),
                    MachineState::Running => next_state.set(MachineState::Paused),
                    _ => (),
                }
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


