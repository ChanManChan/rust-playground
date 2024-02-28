use crate::asset_loader::SceneAssets;
use crate::collision_detection::Collider;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use bevy::app::{App, Plugin, PostStartup, Update};
use bevy::ecs::component::Component;
use bevy::ecs::query::With;
use bevy::ecs::system::Query;
use bevy::input::keyboard::KeyCode;
use bevy::input::ButtonInput;
use bevy::math::{EulerRot, Vec3};
use bevy::prelude::{Commands, Res, Transform};
use bevy::scene::SceneBundle;
use bevy::time::Time;
use bevy::utils::default;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., -20.);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 15.5;
const SPACESHIP_RADIUS: f32 = 5.0;
const MISSLE_SPEED: f32 = 50.;
const MISSLE_FORWARD_SPAWN_SCALAR: f32 = 7.5;
const MISSLE_RADIUS: f32 = 1.0;

#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct SpaceshipMissile;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship).add_systems(
            Update,
            (spaceship_movement_controls, spaceship_weapon_controls),
        );
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            collider: Collider::new(SPACESHIP_RADIUS),
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            model: SceneBundle {
                scene: scene_assets.spaceship.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
        Spaceship,
    ));
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = query.single_mut();
    let mut rotation = 0.;
    let mut roll = 0.;
    let mut movement = 0.;

    if keyboard_input.pressed(KeyCode::KeyD) {
        rotation = -SPACESHIP_ROTATION_SPEED * time.delta_seconds();
        roll = SPACESHIP_ROLL_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        rotation = SPACESHIP_ROTATION_SPEED * time.delta_seconds();
        roll = -SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        movement = -SPACESHIP_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyW) {
        movement = SPACESHIP_SPEED;
    }

    // Rotate around the y-axis
    // ignores the z-axis rotation applied below.
    transform.rotate_y(rotation);

    // Rotate around the local z-axis
    // The rotation is relative to the current rotation
    let current_z = transform.rotation.to_euler(EulerRot::XYZ).2;

    if roll == 0. {
        transform.rotate_local_z(-current_z);
    } else {
        transform.rotate_local_z(-current_z + roll);
    }

    velocity.value = -transform.forward() * movement;
}

fn spaceship_weapon_controls(
    mut commands: Commands,
    query: Query<&Transform, With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    scene_assets: Res<SceneAssets>,
) {
    let transform = query.single();

    if keyboard_input.pressed(KeyCode::Space) {
        commands.spawn((
            MovingObjectBundle {
                collider: Collider::new(MISSLE_RADIUS),
                velocity: Velocity::new(-transform.forward() * MISSLE_SPEED),
                acceleration: Acceleration::new(Vec3::ZERO),
                model: SceneBundle {
                    scene: scene_assets.missles.clone(),
                    transform: Transform::from_translation(
                        transform.translation + -transform.forward() * MISSLE_FORWARD_SPAWN_SCALAR,
                    ),
                    ..default()
                },
            },
            SpaceshipMissile,
        ));
    }
}
