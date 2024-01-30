mod components;
mod systems;

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 500.0;

use bevy::{app::IntoSystemAppConfig, ecs::schedule::{IntoSystemConfig, IntoSystemConfigs, OnEnter, OnExit, OnUpdate}};

use crate::AppState;

use self::systems::*;

use super::SimulationState;

pub struct PlayerPlugin;

impl bevy::prelude::Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
        .add_systems((player_movement, confine_player_movement.after(player_movement), enemy_hit_player, player_hit_star)
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(SimulationState::Running))
        )
        .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}