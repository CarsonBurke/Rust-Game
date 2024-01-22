#![allow(unused)]

use bevy::{
    ecs::schedule::IntoSystemConfigs,
    prelude::{App, Component, Query, Startup, Update},
    window::{WindowPlugin, WindowTheme, Window},
    DefaultPlugins,
};
use bullets::BulletPlugin;
use hello::HelloPlugin;
use structs::Position;
use game_init::game_init;
use players::{player_actions, PlayerPlugin};

pub mod structs;
pub mod startup;
pub mod hello;
pub mod game_init;
pub mod players;
pub mod controls;
pub mod bullets;

fn main() {
    /* let startup = GameStartup; */

    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin, BulletPlugin, PlayerPlugin, controls::ControlsPlugin))
        .add_systems(Startup, game_init)
        /* .add_systems(Update, player_actions) */
        .run();
}
