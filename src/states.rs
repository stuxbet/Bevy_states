use bevy::prelude::*;

#[derive(Debug,Clone,Copy,Default,States,Hash, Eq, PartialEq, Resource )]
pub enum MachineState {
    #[default]
    Idle,
    Running,
    Paused,
    EmergencyShutdown,
    EmergencyIdle,
    Turnoff,
}

pub struct StatePlugin;
//plugin system setup to add to app
impl Plugin for StatePlugin {
    fn build(&self,app: &mut App) {
        app.init_state::<MachineState>();
    }
}

