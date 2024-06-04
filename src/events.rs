use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;

//FIXME:  this makes this system not fully indepenant from the states.rs so I think this should be looked at
use crate::states::MachineState;

// SimpleEvent is just the event struct im using
#[derive(Event)]
struct SimpleEvent {
    pub message: String,
    pub event_type: EventTypes
}
#[derive(Debug)]
enum EventTypes {
    Start,
    Stop,
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
            message: "Stop event sent".to_string(),
            event_type: EventTypes::Stop,
        });
    }
    if keyboard_input.just_pressed(KeyCode::KeyS)  {
        event_writer.send(SimpleEvent {
            message: "Startup detected".to_string(),
            event_type: EventTypes::Start,
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
            message: "Pause pressed".to_string(),
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
        match (state.get(), &event.event_type)  {

            // (MachineState::Idle, EventTypes::Emergency) | (MachineState::Running, EventTypes::Emergency) |(MachineState::Paused, EventTypes::Emergency) => {
            //     next_state.set(MachineState::EmergencyShutdown);
            //     on_enter_emergency();
            // }


            (_, EventTypes::Emergency) => {
                next_state.set(MachineState::EmergencyShutdown);
                on_enter_emergency(&mut next_state);
            }
            //FIXME: this condition must be redefined 
            (MachineState::EmergencyShutdown, EventTypes::Start) => {
                next_state.set(MachineState::EmergencyShutdown);

            }
            (MachineState::Idle, EventTypes::Start) => {
                next_state.set(MachineState::Running);

            }
            (MachineState::Running, EventTypes::PauseButtonHit) => {
                next_state.set(MachineState::Paused);

            }
            (MachineState::Paused, EventTypes::PauseButtonHit) => {
                next_state.set(MachineState::Running);

            }
            (MachineState::Running, EventTypes::Stop) => {
                next_state.set(MachineState::Idle);

            }
            _ => {
                // Invalid transition, do nothing or log an error
                println!("Invalid transition from {:?} with event {:?}", state.get(), event.event_type);
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
//Here is a sample function to define on enter behavior
fn on_enter_emergency(next_state:&mut ResMut<NextState<MachineState>>,
) {
    println!("Entering Emergency State!");
    // Add your emergency behavior here
    //send normal conditions reached when safe to switch to emergency idle

    //this makes it appear like it never reaches emshupdown but it is just redefined before the end of the frame
    next_state.set(MachineState::EmergencyIdle);

}

