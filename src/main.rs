#![allow(unused)]

use bevy::{
    ecs::schedule::IntoSystemConfigs,
    prelude::{App, Component, Query, Startup, Update},
    window::{WindowPlugin, WindowTheme, Window},
    DefaultPlugins,
};
use controls::ControlsPlugin;
use hello::HelloPlugin;
use types::Position;
use game_init::game_init;
use players::{player_actions, PlayerPlugin};

pub mod types;
pub mod startup;
pub mod update;
pub mod hello;
pub mod game_init;
pub mod players;
pub mod controls;

fn main() {
    /* let startup = GameStartup; */

    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin, PlayerPlugin, ControlsPlugin))
        .add_systems(Startup, game_init)
        /* .add_systems(Update, player_actions) */
        .run();
}
