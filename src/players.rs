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

/* pub fn player_actions(time: Res<Time>, mut sprite_pos: Query<(&mut Transform)>) {
    println!("player_actions");

    for (mut transform) in &mut sprite_pos {
        transform.translation.x += 100. * time.delta_seconds();
        transform.translation.y += 100. * time.delta_seconds();
    }
} */

pub fn player_actions(
    time: Res<Time>,
    mut player_positions: Query<(&mut Transform), With<Player>>,
) {
    for (mut transform) in &mut player_positions {
        
        transform.translation.x += 150. * time.delta_seconds();
        transform.translation.y -= 150. * time.delta_seconds();
        
        println!("transform x: {:?}, y: {:?}", transform.translation.x, transform.translation.y);

        /* if transform.translation.y > 200. {
            *logo = Direction::Down;
        } else if transform.translation.y < -200. {
            *logo = Direction::Up;
        } */
    }
}