use bevy::prelude::*;

use crate::asset_loader::GameAssets;
use crate::game::ui::hud::components::*;
use crate::game::ui::hud::styles::*;

pub fn spawn_hud(mut commands: Commands, game_assets: Res<GameAssets>) {
    build_hud(&mut commands, &game_assets);
}

pub fn build_hud(commands: &mut Commands, game_assets: &Res<GameAssets>) -> Entity {
    let hud_entity = commands
        .spawn((
            NodeBundle {
                style: HUD_STYLE,
                ..default()
            },
            HUD {},
        ))
        .with_children(|parent| {
            // LHS
            parent
                .spawn(NodeBundle {
                    style: LHS_STYLE,
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Star Image
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: game_assets.star.clone().into(),
                        ..default()
                    });

                    // Score Text
                    parent.spawn((
                        TextBundle {
                            style: Style { ..default() },
                            text: Text {
                                sections: vec![TextSection::new("0", get_text_style(&game_assets))],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        ScoreText {},
                    ));
                });

            // RHS
            parent
                .spawn(NodeBundle {
                    style: RHS_STYLE,
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Enemy Text
                    parent.spawn((
                        TextBundle {
                            style: Style { ..default() },
                            text: Text {
                                sections: vec![TextSection::new("0", get_text_style(&game_assets))],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        EnemyText {},
                    ));

                    // Enemy Image
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: game_assets.red_ball.clone().into(),
                        ..default()
                    });
                });
        })
        .id();

    hud_entity
}

pub fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<HUD>>) {
    for entity in hud_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
