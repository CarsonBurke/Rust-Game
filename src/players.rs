use bevy::{
    app::{App, Plugin, Update},
    ecs::{system::{Query, Res}, query::With},
    input::{keyboard::KeyCode, Input},
    time::Time,
    transform::components::Transform,
};

use crate::structs::Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        
    }
}