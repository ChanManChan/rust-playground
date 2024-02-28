mod asset_loader;
mod asteroids;
mod camera;
mod collision_detection;
mod debug;
mod despawn;
mod movement;
mod spaceship;

use crate::camera::CameraPlugin;
use crate::debug::DebugPlugin;
use crate::despawn::DespawnPlugin;
use crate::movement::MovementPlugin;
use crate::spaceship::SpaceshipPlugin;
use asset_loader::AssetLoaderPlugin;
use asteroids::AsteroidPlugin;
use bevy::prelude::*;
use collision_detection::CollisionDetectionPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0., 0.15)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 500.0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(DespawnPlugin)
        //        .add_plugins(DebugPlugin)
        .run();
}
