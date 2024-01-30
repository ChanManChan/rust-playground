mod enemy;
mod player;
mod score;
mod star;
mod systems;
mod ui;

use bevy::{app::IntoSystemAppConfig, ecs::schedule::{common_conditions::in_state, IntoSystemConfig, OnEnter, OnExit, States}};
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use ui::GameUIPlugin;
use crate::{events::GameOver, AppState};
use systems::{toggle_simulation, pause_simulation};

use self::systems::resume_simulation;

pub struct GamePlugin;

impl bevy::prelude::Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_state::<SimulationState>()
        .add_event::<GameOver>()
        .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(StarPlugin)
        .add_plugin(GameUIPlugin)
        .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
        .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused
}