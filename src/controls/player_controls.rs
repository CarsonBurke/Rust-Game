use crate::{structs::{Bullet, Player}, constants::{control_keys, player}};
use bevy::{
    asset::AssetServer,
    ecs::{
        query::With,
        system::{Commands, Query, Res},
    },
    input::{keyboard::KeyCode, Input},
    prelude::{App, Plugin, Update},
    sprite::SpriteBundle,
    time::Time,
    transform::components::Transform,
    utils::default,
};

pub struct PlayerControlsPlugin;

impl Plugin for PlayerControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (control_player_movement, control_player_shooting));
    }
}

fn control_player_movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_positions: Query<(&mut Transform), With<Player>>,
) {

    let speed = find_speed(&input);

    if input.pressed(control_keys::MOVE_UP) {
        move_players(&mut player_positions, &time, 0., speed);
    }

    if input.pressed(control_keys::MOVE_DOWN) {
        move_players(&mut player_positions, &time, 0., -speed);
    }

    if input.pressed(control_keys::MOVE_LEFT) {
        move_players(&mut player_positions, &time, -speed, 0.);
    }

    if input.pressed(control_keys::MOVE_RIGHT) {
        move_players(&mut player_positions, &time, speed, 0.);
    }
}

fn find_speed(input: &Res<Input<KeyCode>>) -> f32 {
    if input.pressed(control_keys::BOOST) {
        return player::BOOST_SPEED as f32
    }

    // Otherwise we aren't boosting

    return player::SPEED as f32
}

fn move_players(
    mut player_positions: &mut Query<(&mut Transform), With<Player>>,
    time: &Res<Time>,
    x: f32,
    y: f32,
) {
    for (mut transform) in player_positions {
        transform.translation.x += x * time.delta_seconds();
        transform.translation.y += y * time.delta_seconds();
    }
}

fn control_player_shooting(
    input: Res<Input<KeyCode>>,
    mut player_positions: Query<(&mut Transform), With<Player>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if !input.pressed(control_keys::SHOOT) {
        return;
    }

    // The player is shooting

    for (mut transform) in &mut player_positions {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("player_laser.png"),
                transform: Transform {
                    translation: transform.translation.clone(),
                    ..default()
                },
                ..default()
            },
            Bullet,
        ));
    }
}

fn players_shoot(mut player_positions: &mut Query<(&mut Transform), With<Player>>) {}
