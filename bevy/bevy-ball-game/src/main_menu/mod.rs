use bevy::{app::IntoSystemAppConfig, ecs::schedule::{IntoSystemConfigs, OnEnter, OnExit, OnUpdate}};

use crate::AppState;

use self::systems::{interactions::{interact_with_play_button, interact_with_quit_button}, layout::{despawn_main_menu, spawn_main_menu}};

mod components;
mod styles;
mod systems;

pub struct MainMenuPlugin;

impl bevy::prelude::Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_systems((
                    interact_with_play_button,
                    interact_with_quit_button
                ).in_set(OnUpdate(AppState::MainMenu))
            )
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}
