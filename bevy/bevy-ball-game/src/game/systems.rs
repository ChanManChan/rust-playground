use bevy::ecs::schedule::NextState;
use bevy::ecs::system::ResMut;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::{Commands, Res, Input, State};

use super::SimulationState;

pub fn pause_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Running);
}

pub fn toggle_simulation(mut commands: Commands, keyboard_input: Res<Input<KeyCode>>, simulation_state: Res<State<SimulationState>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if *simulation_state.get() == SimulationState::Running {
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
            println!("Simulation Paused.")
        } else if *simulation_state.get() == SimulationState::Paused {
            commands.insert_resource(NextState(Some(SimulationState::Running)));
            println!("Simulation Running.")
        }
    }    
}