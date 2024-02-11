use crate::{constants::{bullet, bullet_player, control_keys, player}, structs::{Bullet, Player}, utils::Utils};
use bevy::{
    app::Startup, asset::AssetServer, ecs::{
        event::EventReader, query::With, system::{Commands, Query, Res}
    }, input::{keyboard::KeyCode, Input}, prelude::{App, Plugin, Update, Vec2, Vec3}, render::camera::Camera, sprite::SpriteBundle, time::Time, transform::components::{GlobalTransform, Transform}, utils::default, window::{CursorMoved, PrimaryWindow, Window}
};

pub struct PlayerControlsPlugin;

impl Plugin for PlayerControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (control_player_movement, control_player_shooting))
        /* .add_event(cursor_events)*/;
    }
}

fn control_player_movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_positions: Query<(&mut Transform, &mut Player), With<Player>>,
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
    mut player_positions: &mut Query<(&mut Transform, &mut Player), With<Player>>,
    time: &Res<Time>,
    x: f32,
    y: f32,
) {
    for (mut transform, player) in player_positions {
        transform.translation.x += x * time.delta_seconds();
        transform.translation.y += y * time.delta_seconds();
    }
}

fn control_player_shooting(
    input: Res<Input<KeyCode>>,
    mut player_positions: Query<(&mut Transform, &mut Player), With<Player>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window, With<PrimaryWindow>>
) {
    if !input.pressed(control_keys::SHOOT) {
        return;
    }

    let cursor_pos = find_cursor_position(cameras, windows);
    let player_tuple = player_positions.single();
    let player_pos = player_tuple.0.translation;
    
    let angle = Utils::find_angle(cursor_pos.x, cursor_pos.y, player_pos.x, player_pos.y);
    println!("angle {}", angle);

    // The player is shooting

    for (mut transform, player) in &mut player_positions {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(bullet_player::ASSET_PATH),
                transform: Transform {
                    translation: Vec3 {
                        x: cursor_pos.x,
                        y: cursor_pos.y,
                        z: bullet::Z_POS,
                    },
                    ..default()
                },
                ..default()
            },
            Bullet, 
        ));
    }
}

fn players_shoot(mut player_positions: &mut Query<(&mut Transform), With<Player>>) {}

fn find_cursor_position(
    cameras: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window, With<PrimaryWindow>>,
) -> Vec2 {

    let (camera, camera_transform) = cameras.single();

    let Some(cursor_absolute_pos) = windows.single().cursor_position() else {
        return Vec2 {
            x: 0.,
            y: 0.,
        };
    };

    // Calculate a world position based on the cursor's position.
    let Some(cursor_pos) = camera.viewport_to_world_2d(camera_transform, cursor_absolute_pos) else {
        return Vec2 {
            x: 0.,
            y: 0.,   
        };
    };

    return cursor_pos
}

fn cursor_events(
    mut cursor_evr: EventReader<CursorMoved>,
) {
    for ev in cursor_evr.read() {
        println!(
            "New cursor position: X: {}, Y: {}, in Window ID: {:?}",
            ev.position.x, ev.position.y, ev.window
        );
    }
}