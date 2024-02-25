mod debug;
mod movement;
mod spaceship;
mod camera;

use bevy::prelude::*;
use crate::camera::CameraPlugin;
use crate::debug::DebugPlugin;
use crate::movement::MovementPlugin;
use crate::spaceship::SpaceshipPlugin;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0., 0.15)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 500.0
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .run();
}

