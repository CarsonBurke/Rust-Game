use std::f32::consts::PI;

use crate::{
    constants::{player, window},
    structs::{Gun, Player},
};
use ::winit::window::Icon;
use bevy::{
    animation::prelude,
    asset::{AssetServer, Handle},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::system::{Commands, NonSend, Query, Res, ResMut},
    math::Vec3,
    render::{texture::Image, view::WindowSurfaces},
    sprite::SpriteBundle,
    time::Time,
    transform::components::Transform,
    utils::default,
    window::Window,
    winit::{self, WinitWindows},
};

pub struct GameInit;

const PLAYER_SPRITE: &str = "player.png";
struct GameTextures {
    player: Handle<Image>,
}

pub fn game_init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: NonSend<WinitWindows>,
    time: Res<Time>,
) {
    commands.spawn(Camera2dBundle::default());

    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(window::ASSET_PATH)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    // do it for all windows
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
        window.set_title(window::PRIMARY_WINDOW_TITLE);
    }
}
