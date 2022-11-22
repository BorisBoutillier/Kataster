use crate::prelude::*;

#[derive(Debug, Resource)]
pub struct GameAssets {
    pub font: Handle<Font>,
    pub laser_texture: Handle<Image>,
    pub laser_audio: Handle<AudioSource>,
    pub meteor_big: Handle<Image>,
    pub meteor_med: Handle<Image>,
    pub meteor_small: Handle<Image>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        font: asset_server.load("kenvector_future.ttf"),
        laser_texture: asset_server.load("laserRed07.png"),
        laser_audio: asset_server.load("sfx_laser1.ogg"),
        meteor_big: asset_server.load("meteorBrown_big1.png"),
        meteor_med: asset_server.load("meteorBrown_med1.png"),
        meteor_small: asset_server.load("meteorBrown_small1.png"),
    });
}
