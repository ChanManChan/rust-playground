use bevy::{
    app::{App, Plugin, Startup},
    asset::{AssetServer, Handle},
    audio::AudioSource,
    ecs::system::{Res, ResMut, Resource},
    render::texture::Image,
    text::Font,
};

#[derive(Debug, Default, Resource)]
pub struct GameAssets {
    pub red_ball: Handle<Image>,
    pub blue_ball: Handle<Image>,
    pub star: Handle<Image>,
    pub font: Handle<Font>,
    pub explosion: Handle<AudioSource>,
    pub pluck1: Handle<AudioSource>,
    pub pluck2: Handle<AudioSource>,
    pub lazer: Handle<AudioSource>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut game_assets: ResMut<GameAssets>, asset_server: Res<AssetServer>) {
    *game_assets = GameAssets {
        red_ball: asset_server.load("sprites/ball_red_large.png"),
        blue_ball: asset_server.load("sprites/ball_blue_large.png"),
        star: asset_server.load("sprites/star.png"),
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        explosion: asset_server.load("audio/explosionCrunch_000.ogg"),
        pluck1: asset_server.load("audio/pluck_001.ogg"),
        pluck2: asset_server.load("audio/pluck_002.ogg"),
        lazer: asset_server.load("audio/laserLarge_000.ogg"),
    }
}
