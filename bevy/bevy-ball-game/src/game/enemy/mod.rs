pub mod components;
pub mod resources;
mod systems;

use bevy::app::IntoSystemAppConfig;
use bevy::ecs::schedule::{IntoSystemConfigs, OnEnter, OnExit, OnUpdate};

use crate::AppState;

use self::resources::EnemySpawnTimer;
use self::systems::*;

use super::SimulationState;

pub const NUMBER_0F_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;

pub struct EnemyPlugin;

impl bevy::prelude::Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .init_resource::<EnemySpawnTimer>()
        .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
        .add_systems((enemy_movement, update_enemy_direction, confine_enemy_movement, tick_enemy_spawn_timer, spawn_enemies_over_time)
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(SimulationState::Running))
        )
        .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
    }
}