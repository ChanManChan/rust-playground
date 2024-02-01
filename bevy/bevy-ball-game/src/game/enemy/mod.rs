pub mod components;
pub mod resources;
mod systems;

use bevy::app::Update;
use bevy::ecs::schedule::common_conditions::in_state;
use bevy::ecs::schedule::{IntoSystemConfigs, OnEnter, OnExit};

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
        .add_systems(OnEnter(AppState::Game), spawn_enemies)
        .add_systems(Update, (enemy_movement, update_enemy_direction.after(enemy_movement), confine_enemy_movement.after(update_enemy_direction), tick_enemy_spawn_timer, spawn_enemies_over_time)
            .run_if(in_state(AppState::Game))
            .run_if(in_state(SimulationState::Running))
        )
        .add_systems(OnExit(AppState::Game), despawn_enemies);
    }
}