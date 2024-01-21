use crate::types::Direction;
use bevy::{
    app::{App, Plugin, Update},
    ecs::system::{Query, Res},
    input::{keyboard::KeyCode, Input},
    time::Time,
    transform::components::Transform,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (player_actions, move_player));
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
    mut sprite_position: Query<(&mut Direction, &mut Transform)>,
) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
        }

        if transform.translation.y > 200. {
            *logo = Direction::Down;
        } else if transform.translation.y < -200. {
            *logo = Direction::Up;
        }
    }
}

pub fn move_player(input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        println!("pressed {:?}", KeyCode::Space);
    }
}
