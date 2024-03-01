use bevy::prelude::*;

use crate::{score::Score, state::GameState};

#[derive(Component)]
pub struct GameOverMenu;

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), spawn_gameover_menu)
            .add_systems(OnExit(GameState::GameOver), despawn_gameover_menu);
    }
}

fn spawn_gameover_menu(mut commands: Commands, score: Res<Score>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                z_index: ZIndex::Local(2),
                ..default()
            },
            GameOverMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(300.),
                        height: Val::Px(300.),
                        row_gap: Val::Px(8.),
                        column_gap: Val::Px(8.),
                        ..default()
                    },
                    background_color: Color::rgba(0.25, 0.25, 0.25, 0.5).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Game Over",
                                TextStyle {
                                    font_size: 64.,
                                    color: Color::rgb(1., 1., 1.),
                                    ..default()
                                },
                            )],
                            ..default()
                        },
                        ..default()
                    });

                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                format!("Final Score: {}", score.value),
                                TextStyle {
                                    font_size: 16.,
                                    color: Color::rgb(1., 1., 1.),
                                    ..default()
                                },
                            )],
                            ..default()
                        },
                        ..default()
                    });

                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Restart: [Space]",
                                TextStyle {
                                    font_size: 16.,
                                    color: Color::rgb(1., 1., 1.),
                                    ..default()
                                },
                            )],
                            ..default()
                        },
                        ..default()
                    });
                });
        });
}

fn despawn_gameover_menu(
    mut commands: Commands,
    gameover_menu_query: Query<Entity, With<GameOverMenu>>,
) {
    if let Ok(gameover_menu_entity) = gameover_menu_query.get_single() {
        commands.entity(gameover_menu_entity).despawn_recursive();
    }
}
