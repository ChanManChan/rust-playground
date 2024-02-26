mod asteroids;
mod camera;
mod debug;
mod movement;
mod spaceship;

use crate::camera::CameraPlugin;
use crate::debug::DebugPlugin;
use crate::movement::MovementPlugin;
use crate::spaceship::SpaceshipPlugin;
use asteroids::AsteroidPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0., 0.15)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 500.0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .run();
}
