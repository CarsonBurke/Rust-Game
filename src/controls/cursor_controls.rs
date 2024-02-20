use bevy::{app::{App, Plugin, Update}, ecs::system::Query, transform::components::Transform};

use crate::structs::Unit;

use super::CameraControlsPlugin;

pub struct CursorControlsPlugin;

impl Plugin for CursorControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, select_units);
    }
}

fn select_units(mut units: Query<(&mut Unit, &mut Transform)>) {

    for (unit, unit_transform) in &mut units {
/*         if unit_transform {
            continue;
        }

 */
    }
}