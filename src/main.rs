use bevy::prelude::*;
use bevy::window::{
    WindowMode,
    WindowId
};
use bevy::winit::WinitWindows;
use winit::window::Icon;

mod player;
mod enemy;
mod constants;
mod components;
mod resources;
mod background;

use constants::*;
use resources::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.11, 0.11, 0.11)))
        .insert_resource(WindowDescriptor {
            title: "hShip ][: the last voyage".to_string(),
            width: 1366.,
            height: 768.,
            mode: WindowMode::Fullscreen,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(background::BackgroundPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
    winit_windows: NonSend<WinitWindows>
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_visibility(false);
    commands.insert_resource(WinSize { w: window.width(), h: window.height() });

    let window = winit_windows.get_window(WindowId::primary()).unwrap();
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
    window.set_window_icon(Some(icon));

    commands.insert_resource(GameTextures {
        player: asset_server.load(PLAYER_SPRITE_PATH),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE_PATH),
        enemy1: asset_server.load(ENEMY1_SPRITE_PATH),
        enemy2: asset_server.load(ENEMY2_SPRITE_PATH),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE_PATH),
        background: asset_server.load(BACKGROUND_SPRITE_PATH)
    });
}
