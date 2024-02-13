#![allow(unused)]

use bevy::{
    ecs::schedule::IntoSystemConfigs,
    prelude::{App, Component, Query, Startup, Update},
    window::{WindowPlugin, WindowTheme, Window},
    DefaultPlugins,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game::GamePlugin;
use structs::Position;

pub mod structs;
pub mod game_init;
pub mod players;
pub mod controls;
pub mod bullets;
pub mod game;
pub mod constants;
pub mod tiles;
pub mod cursor;
pub mod utils;

fn main() {
    /* let startup = GameStartup; */

    App::new()
        .add_plugins((DefaultPlugins, GamePlugin))
        //.add_plugins(WorldInspectorPlugin::new())
        .run();
}
