use bevy::app::{App, Plugin};

use super::PlayerPlugin;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin));
    }
}