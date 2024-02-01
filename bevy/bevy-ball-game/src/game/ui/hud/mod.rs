mod components;
mod styles;
mod systems;

use systems::layout::*;

use crate::game::ui::hud::systems::updates::{update_enemy_text, update_score_text};
use crate::AppState;
use bevy::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), spawn_hud)
            .add_systems(Update, (update_score_text, update_enemy_text).run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), despawn_hud);
    }
}