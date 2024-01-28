pub mod resources;
mod systems;

use bevy::app::IntoSystemAppConfig;
use bevy::ecs::schedule::common_conditions::in_state;
use bevy::ecs::schedule::IntoSystemConfig;
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
        .add_system(insert_score.in_schedule(OnEnter(AppState::Game)))
        .add_system(update_score.run_if(in_state(AppState::Game)))
        .add_system(update_high_scores)
        .add_system(high_scores_updated)
        .add_system(remove_score.in_schedule(OnExit(AppState::Game)));
    }
}