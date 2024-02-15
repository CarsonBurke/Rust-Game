use crate::buildings::{BuildingsPlugin, UnitFactoryPlugin};
use crate::cursor::CursorPlugin;
use crate::units::UnitsPlugin;
use crate::{bullets::BulletPlugin, tiles::TilesPlugin};
use crate::controls;
use crate::game_init::game_init;
use bevy::app::{App, Plugin, Startup};

use crate::structs::{Bullet, Player};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BulletPlugin, controls::ControlsPlugin, TilesPlugin, CursorPlugin, BuildingsPlugin, UnitsPlugin))
            .add_systems(Startup, game_init);
    }
}

fn spawn_player() {

}