use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;


// Define a simple event
#[derive(Event)]
struct SimpleEvent {
    pub message: String,
    pub event_type: EventTypes
}
#[derive(Event,Debug)]
enum EventTypes {
    ShitHitsTheFan,
    BigFire,
    Explosion,
    MiscComputerIssue
}

//System to send SimpleEvent periodically
fn send_event_system(
    mut event_writer: EventWriter<SimpleEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,

) {
    if keyboard_input.pressed(KeyCode::KeyA)  {
        event_writer.send(SimpleEvent {
            message: "A pressed".to_string(),
            event_type: EventTypes::BigFire,
        });
    }
    if keyboard_input.pressed(KeyCode::KeyS)  {
        event_writer.send(SimpleEvent {
            message: "S pressed".to_string(),
            event_type: EventTypes::ShitHitsTheFan,
        });
    }
    if keyboard_input.pressed(KeyCode::KeyD)  {
        event_writer.send(SimpleEvent {
            message: "D pressed".to_string(),
            event_type: EventTypes::Explosion,
        });
    }
    if keyboard_input.pressed(KeyCode::KeyW)  {
        event_writer.send(SimpleEvent {
            message: "W pressed".to_string(),
            event_type: EventTypes::MiscComputerIssue,
        });
    }
}


// System to handle SimpleEvent
fn handle_event_system(mut event_reader: EventReader<SimpleEvent>) {
    for event in event_reader.read() {
        info!("{}:{:?}",event.message, event.event_type);
    }
}

// Define the plugin
pub struct SimpleEventPlugin;

impl Plugin for SimpleEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SimpleEvent>()
            .add_systems(Update,send_event_system)
            .add_systems(Update,handle_event_system);
    }
}


