use bevy::{app::{App, Plugin, Update}, ecs::{system::{Res, Query}, query::With}, input::{Input, keyboard::KeyCode}, transform::components::Transform, time::Time};

use crate::types::Player;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (control_players));
    }
}

pub fn control_players(input: Res<Input<KeyCode>>, time: Res<Time>, mut player_positions: Query<(&mut Transform), With<Player>>) {

    if input.pressed(KeyCode::W) {
        println!("pressed {:?}", KeyCode::W);

        move_players(&mut player_positions, &time, 0., 100.);
    }
    
    if input.pressed(KeyCode::S) {
        println!("pressed {:?}", KeyCode::S);

        move_players(&mut player_positions, &time, 0., -100.);
    }

    if input.pressed(KeyCode::A) {
        println!("pressed {:?}", KeyCode::A);

        move_players(&mut player_positions, &time, -100., 0.);
    }

    if input.pressed(KeyCode::D) {
        println!("pressed {:?}", KeyCode::D);

        move_players(&mut player_positions, &time, 100., 0.);
    }
}

fn move_players(mut player_positions: &mut Query<(&mut Transform), With<Player>>, time: &Res<Time>, x: f32, y: f32) {
    for (mut transform) in player_positions {
        transform.translation.x += x * time.delta_seconds();
        transform.translation.y += y * time.delta_seconds();
    }
}