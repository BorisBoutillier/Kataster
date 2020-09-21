use super::state::*;
use bevy::prelude::*;

pub fn main_menu(
    mut commands: Commands,
    runstate: ResMut<RunState>,
    asset_server: Res<AssetServer>,
) {
    if runstate.enter && runstate.next == Some(GameState::MainMenu) {
        let font_handle = asset_server.load("assets/kenvector_future.ttf").unwrap();
        commands
            // 2d camera
            .spawn(UiCameraComponents::default())
            // texture
            .spawn(TextComponents {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    ..Default::default()
                },
                text: Text {
                    value: "TEST".to_string(),
                    font: font_handle,
                    style: TextStyle {
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                },
                ..Default::default()
            });
    }
}
