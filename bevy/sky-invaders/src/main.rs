mod asset_loader;
mod asteroids;
mod camera;
mod collision_detection;
mod debug;
mod despawn;
mod gameover_menu;
mod health;
mod movement;
mod schedule;
mod score;
mod spaceship;
mod state;

use crate::camera::CameraPlugin;
// use crate::debug::DebugPlugin;
use crate::despawn::DespawnPlugin;
use crate::movement::MovementPlugin;
use crate::spaceship::SpaceshipPlugin;
use asset_loader::AssetLoaderPlugin;
use asteroids::AsteroidPlugin;
use bevy::prelude::*;
use collision_detection::CollisionDetectionPlugin;
use gameover_menu::GameOverMenuPlugin;
use schedule::SchedulePlugin;
use score::ScorePlugin;
use state::StatePlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0., 0.15)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 500.0,
        })
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "__wasm__skyinvaders".to_string(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some("#game-wrapper-canvas".to_string()),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(CameraPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(DespawnPlugin)
        //.add_plugins(DebugPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(StatePlugin)
        .add_plugins(ScorePlugin)
        .add_plugins(GameOverMenuPlugin)
        .run();
}
