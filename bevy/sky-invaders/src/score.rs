use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        change_detection::DetectChanges,
        schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter},
        system::Commands,
    },
    prelude::{Res, Resource},
};

use crate::state::GameState;

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Self {
        Self { value: 0 }
    }
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(OnEnter(GameState::InGame), insert_score)
            .add_systems(Update, print_score.run_if(in_state(GameState::InGame)));
    }
}

fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

fn print_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}
