use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;

//FIXME:  this makes this system not fully indepenant from the states.rs so I think this should be looked at
use crate::states::MachineState;

// SimpleEvent is just the event struct im using
#[derive(Event)]
pub struct SimpleEvent {
    pub message: String,
    pub event_type: EventTypes
}
#[derive(Debug)]
pub enum EventTypes {
    Start,
    Stop,
    Emergency,
    PauseButtonHit,
    Power
}

//this is a wrapper function so that the event writer can be used in the send one event api call without the outside program needing access to the event writer
struct MyEventWriterResource<'a> {
    writer: Option<EventWriter<'a, SimpleEvent>>,
}

impl Default for MyEventWriterResource<'_> {
    fn default() -> Self {
        Self { writer: None }
    }
}


// pub fn send_one_event_system(
//     mut event_writer: EventWriter<SimpleEvent>,
//     state_type: EventTypes

// ) {
//     event_writer.send(SimpleEvent{
//         message: "Single event went through".to_string(),
//         event_type: state_type

//     });
// }


pub fn send_simple_event(event_writer: &mut EventWriter<SimpleEvent>, new_state: EventTypes) {
    event_writer.send(SimpleEvent {
        message: "Single event went through".to_string(),
        event_type: new_state,
    });
}






pub fn send_event_system(
    mut event_writer: EventWriter<SimpleEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,

) {

//TODO: in implementation these keyboard imputs will instead be if statements watching for stimuli either from websocket/webserver (startbutton etc) or sensor data (fire detected etc)

for key_code in keyboard_input.get_just_pressed() {
    let event = match key_code {
        KeyCode::KeyA => Some(SimpleEvent {
            message: "Stop event sent".to_string(),
            event_type: EventTypes::Stop,
        }),
        KeyCode::KeyS => Some(SimpleEvent {
            message: "Startup detected".to_string(),
            event_type: EventTypes::Start,
        }),
        KeyCode::KeyE => Some(SimpleEvent {
            message: "Emergency condition found sounding the alarm".to_string(),
            event_type: EventTypes::Emergency,
        }),
        KeyCode::KeyP => Some(SimpleEvent {
            message: "Pause pressed".to_string(),
            event_type: EventTypes::PauseButtonHit,
        }),
        KeyCode::Space => Some(SimpleEvent {
            message: "Power Off".to_string(),
            event_type: EventTypes::Power,
        }),
        _ => None,
    };

    if let Some(event) = event {
        event_writer.send(event);
    }
};
    


}

    
// System to handle SimpleEvents and change state accordingly
pub fn handle_event_system(
    mut next_state: ResMut<NextState<MachineState>>,
    mut event_reader: EventReader<SimpleEvent>,
    state: Res<State<MachineState>>
) {
    for event in event_reader.read() {
        /*
        this match statement is where you would impement either single events triggering 
        statechanges may be able to add in parameters that take sensor data to check for emergency conditions
        */
        match (state.get(), &event.event_type)  {


            (_, EventTypes::Emergency) => {
                //this may be a place to put something that stops a spam of Eshut events from disrupting Eshut procedure
                next_state.set(MachineState::EmergencyShutdown);
                println!("got here");
                on_enter_emergency(&mut next_state);
            }
            //TODO: this condition must be redefined before a robot is  
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
            (MachineState::Running, EventTypes::Power)|(MachineState::Idle, EventTypes::Power)|(MachineState::Paused, EventTypes::Power) => {
                next_state.set(MachineState::Turnoff);

            }
            _ => {
                // undefined/invalid transition behavior
                println!("Invalid transition from {:?} with event {:?}", state.get(), event.event_type);
            }
        }
// just debug lines, not neccesary
        info!("{}:{:?}",event.message, event.event_type);
        info!("state{:?}",state);
    }
}

pub struct SimpleEventPlugin;

impl Plugin for SimpleEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SimpleEvent>()
            .add_systems(Update,send_event_system)
            //.add_systems(send_event_system)
            //.add_systems(Update,send_one_event_system)
            .add_systems(Update,handle_event_system);

    }
}


//Here is a sample function to define on enter behavior
pub fn on_enter_emergency(next_state:&mut ResMut<NextState<MachineState>>,
) {
    println!("Entering Emergency State!");
    //TODO: Add your emergency behavior here
    //send normal conditions reached when safe to switch to emergency idle

    //this makes it appear like it never reaches emshupdown but it is just redefined before the end of the frame
    next_state.set(MachineState::EmergencyIdle);

}