use bevy::{audio::Volume, prelude::*};

use crate::{
    asset_loader::SceneAssets, asteroids::ASTEROID_ID, health::Health, schedule::InGameSet,
    score::Score, state::GameState,
};

const DESPAWN_DISTANCE: f32 = 100.;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (despawn_far_away_entities, despawn_dead_entities).in_set(InGameSet::DespawnEntities),
        )
        .add_systems(OnEnter(GameState::GameOver), despawn_all_entities);
    }
}

fn despawn_far_away_entities(mut commands: Commands, query: Query<(Entity, &GlobalTransform)>) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);
        // Entity is far away from the camera's viewport (ie. origin)
        if distance > DESPAWN_DISTANCE {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_dead_entities(
    mut commands: Commands,
    query: Query<(Entity, &'static Health)>,
    mut score: ResMut<Score>,
    scene_assets: Res<SceneAssets>,
) {
    for (entity, health) in query.iter() {
        if health.value <= 0. {
            if health.id == ASTEROID_ID {
                if health.value == 0. {
                    score.value += 1;
                }

                commands.spawn(AudioBundle {
                    source: scene_assets.explosion.clone(),
                    settings: PlaybackSettings::ONCE.with_volume(Volume::new(0.5)),
                });
            }

            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_all_entities(mut commands: Commands, query: Query<(Entity, &'static Health)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
