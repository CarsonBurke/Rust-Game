use bevy::{prelude::{Plugin, App, Update}, ecs::{system::{Res, Query, Commands}, query::With}, input::{Input, keyboard::KeyCode}, time::Time, transform::components::Transform, asset::AssetServer, utils::default, sprite::SpriteBundle};
use crate::structs::{Player, Bullet};

pub struct PlayerControlsPlugin;

impl Plugin for PlayerControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (control_player_movement, control_player_shooting));
    }
}


fn control_player_movement(input: Res<Input<KeyCode>>, time: Res<Time>, mut player_positions: Query<(&mut Transform), With<Player>>) {

    if input.pressed(KeyCode::W) {

        move_players(&mut player_positions, &time, 0., 100.);
    }
    
    if input.pressed(KeyCode::S) {

        move_players(&mut player_positions, &time, 0., -100.);
    }

    if input.pressed(KeyCode::A) {

        move_players(&mut player_positions, &time, -100., 0.);
    }

    if input.pressed(KeyCode::D) {

        move_players(&mut player_positions, &time, 100., 0.);
    }
}

fn move_players(mut player_positions: &mut Query<(&mut Transform), With<Player>>, time: &Res<Time>, x: f32, y: f32) {
    for (mut transform) in player_positions {
        transform.translation.x += x * time.delta_seconds();
        transform.translation.y += y * time.delta_seconds();
    }
}

fn control_player_shooting(input: Res<Input<KeyCode>>, mut player_positions: Query<(&mut Transform), With<Player>>, mut commands: Commands, asset_server: Res<AssetServer>) {
    if !input.pressed(KeyCode::Space) {
        return
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

fn players_shoot(mut player_positions: &mut Query<(&mut Transform), With<Player>>) {
    
}