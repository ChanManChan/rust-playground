use std::vec;

use crate::main_menu::components::{MainMenu, PlayButton};
use crate::{
    asset_loader::GameAssets,
    main_menu::{
        components::QuitButton,
        styles::{
            get_button_text_style, get_title_text_style, BUTTON_STYLE, IMAGE_STYLE,
            MAIN_MENU_STYLE, NORMAL_BUTTON_COLOR, TITLE_STYLE,
        },
    },
};
use bevy::{
    ecs::{
        entity::Entity,
        query::With,
        system::{Query, Res},
    },
    hierarchy::{BuildChildren, DespawnRecursiveExt},
    prelude::{default, Commands},
    text::{Text, TextAlignment, TextSection},
    ui::node_bundles::{ButtonBundle, ImageBundle, NodeBundle, TextBundle},
};

pub fn spawn_main_menu(mut commands: Commands, game_assets: Res<GameAssets>) {
    build_main_menu(&mut commands, &game_assets);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive(); // despawns current entity and all its children
    }
}

pub fn build_main_menu(commands: &mut Commands, game_assets: &Res<GameAssets>) -> Entity {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: MAIN_MENU_STYLE,
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            // Title
            parent
                .spawn(NodeBundle {
                    style: TITLE_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    // Image 1
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: game_assets.blue_ball.clone().into(),
                        ..default()
                    });

                    // Text
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Bally",
                                get_title_text_style(&game_assets),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });

                    // Image 2
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: game_assets.red_ball.clone().into(),
                        ..default()
                    });
                });

            // Play Button
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    PlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Play",
                                get_button_text_style(&game_assets),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });

            // Quit Button
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    QuitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Quit",
                                get_button_text_style(&game_assets),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();

    main_menu_entity
}
