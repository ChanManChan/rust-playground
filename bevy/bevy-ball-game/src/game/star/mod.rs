pub mod components;
pub mod resources;
mod systems;

pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;

use bevy::app::Update;
use bevy::ecs::schedule::common_conditions::in_state;
use bevy::ecs::schedule::IntoSystemConfigs;
use bevy::ecs::schedule::OnEnter;
use bevy::ecs::schedule::OnExit;

use crate::AppState;

use self::resources::*;
use self::systems::*;

use super::SimulationState;

pub struct StarPlugin;

impl bevy::prelude::Plugin for StarPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .init_resource::<StarSpawnTimer>()
        .add_systems(OnEnter(AppState::Game), spawn_stars)
        .add_systems(Update, (tick_star_spawn_timer, spawn_stars_over_time)
            .run_if(in_state(AppState::Game))
            .run_if(in_state(SimulationState::Running))
        )
        .add_systems(OnExit(AppState::Game), despawn_stars);
    }
}