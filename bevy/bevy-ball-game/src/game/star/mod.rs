pub mod components;
pub mod resources;
mod systems;

pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;

use bevy::app::IntoSystemAppConfig;
use bevy::ecs::schedule::IntoSystemConfigs;
use bevy::ecs::schedule::OnEnter;
use bevy::ecs::schedule::OnExit;
use bevy::ecs::schedule::OnUpdate;

use crate::AppState;

use self::resources::*;
use self::systems::*;

use super::SimulationState;

pub struct StarPlugin;

impl bevy::prelude::Plugin for StarPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .init_resource::<StarSpawnTimer>()
        .add_system(spawn_stars.in_schedule(OnEnter(AppState::Game)))
        .add_systems((tick_star_spawn_timer, spawn_stars_over_time)
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(SimulationState::Running))
        )
        .add_system(despawn_stars.in_schedule(OnExit(AppState::Game)));
    }
}