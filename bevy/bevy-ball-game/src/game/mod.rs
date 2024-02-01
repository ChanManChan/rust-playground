mod enemy;
mod player;
mod score;
mod star;
mod systems;
mod ui;

use bevy::{app::Update, ecs::schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter, OnExit, States}};
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
        .add_systems(OnEnter(AppState::Game), pause_simulation)
        .add_plugins((EnemyPlugin, PlayerPlugin, ScorePlugin, StarPlugin, GameUIPlugin))
        .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
        .add_systems(OnExit(AppState::Game), resume_simulation);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused
}