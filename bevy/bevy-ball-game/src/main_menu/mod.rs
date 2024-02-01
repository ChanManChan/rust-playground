use bevy::{app::Update, ecs::schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter, OnExit}};

use crate::AppState;

use self::systems::{interactions::{interact_with_play_button, interact_with_quit_button}, layout::{despawn_main_menu, spawn_main_menu}};

mod components;
mod styles;
mod systems;

pub struct MainMenuPlugin;

impl bevy::prelude::Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(Update, 
                (interact_with_play_button, interact_with_quit_button).run_if(in_state(AppState::MainMenu))
            )
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}
