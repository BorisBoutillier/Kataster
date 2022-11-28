use crate::prelude::*;

#[derive(Debug, Resource)]
pub struct GameAssets {
    pub font: Handle<Font>,
    pub laser_texture: Handle<Image>,
    pub meteor_big: Handle<Image>,
    pub meteor_med: Handle<Image>,
    pub meteor_small: Handle<Image>,
    pub ship_life: UiImage,
    pub player_ship: Handle<Image>,
    pub ship_dead_texture: Handle<Image>,
    pub ship_contact_texture: Handle<Image>,
    pub asteroid_dead_texture: Handle<Image>,
}
#[derive(Debug, Resource)]
pub struct AudioAssets {
    pub laser_trigger: Handle<AudioSource>,
    pub ship_explosion: Handle<AudioSource>,
    pub ship_contact: Handle<AudioSource>,
    pub asteroid_explosion: Handle<AudioSource>,
}

#[derive(Debug, Resource)]
pub struct UiAssets {
    pub font: Handle<Font>,
    pub ship_life: UiImage,
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
        meteor_big: asset_server.load("meteorBrown_big1.png"),
        meteor_med: asset_server.load("meteorBrown_med1.png"),
        meteor_small: asset_server.load("meteorBrown_small1.png"),
        ship_life: asset_server.load("playerLife1_red.png").into(),
        player_ship: asset_server.load("playerShip2_red.png"),
        ship_dead_texture: asset_server.load("explosion01.png"),
        ship_contact_texture: asset_server.load("explosion01.png"),
        asteroid_dead_texture: asset_server.load("flash00.png"),
    });
    commands.insert_resource(AudioAssets {
        laser_trigger: asset_server.load("sfx_laser1.ogg"),
        ship_explosion: asset_server.load("Explosion_ship.ogg"),
        ship_contact: asset_server.load("Explosion.ogg"),
        asteroid_explosion: asset_server.load("Explosion.ogg"),
    });
    commands.insert_resource(UiAssets {
        font: asset_server.load("kenvector_future.ttf"),
        ship_life: asset_server.load("playerLife1_red.png").into(),
    });
}
