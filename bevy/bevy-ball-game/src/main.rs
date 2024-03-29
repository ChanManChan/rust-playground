mod asset_loader;
mod events;
mod game;
mod main_menu;
mod systems;

use asset_loader::AssetLoaderPlugin;
use bevy::window::PresentMode;
use bevy::{
    app::{PluginGroup, Startup, Update},
    asset::{AssetMetaCheck, AssetPlugin},
    ecs::schedule::States,
    prelude::{default, App, DefaultPlugins},
    window::{Window, WindowPlugin},
};
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "__wasm__bally".to_string(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        focused: true,
                        present_mode: PresentMode::AutoVsync,
                        canvas: Some("#game-wrapper-canvas".to_string()),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_state::<AppState>()
        .add_plugins((AssetLoaderPlugin, GamePlugin, MainMenuPlugin))
        .add_systems(Startup, spawn_camera)
        .add_systems(
            Update,
            (
                transition_to_game_state,
                transition_to_main_menu_state,
                exit_game,
                handle_game_over,
            ),
        )
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
