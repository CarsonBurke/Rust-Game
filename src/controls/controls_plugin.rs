use bevy::{app::{App, Plugin, Update}, ecs::{system::{Res, Query}, query::With}, input::{Input, keyboard::KeyCode}, transform::components::Transform, time::Time};

use crate::{structs::Player, controls::PlayerControlsPlugin};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerControlsPlugin);
    }
}