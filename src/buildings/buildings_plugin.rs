use bevy::app::{App, Plugin, Update};

use super::{ScoutFactoryPlugin, UnitFactoryPlugin};

pub struct BuildingsPlugin;

impl Plugin for BuildingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UnitFactoryPlugin, ScoutFactoryPlugin));
    }
}
