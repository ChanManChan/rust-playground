pub mod resources;
mod systems;

use bevy::app::Update;
use bevy::ecs::schedule::common_conditions::in_state;
use bevy::ecs::schedule::IntoSystemConfigs;
use bevy::ecs::schedule::OnEnter;
use bevy::ecs::schedule::OnExit;

use crate::AppState;

use self::systems::*;
use self::resources::*;

pub struct ScorePlugin;

impl bevy::prelude::Plugin for ScorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .init_resource::<HighScores>()
        .add_systems(OnEnter(AppState::Game), insert_score)
        .add_systems(Update, update_score.run_if(in_state(AppState::Game)))
        .add_systems(Update, (update_high_scores, high_scores_updated))
        .add_systems(OnExit(AppState::Game), remove_score);
    }
}