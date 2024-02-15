use std::f32::consts::PI;

use crate::{constants::{bullet, bullet_player, control_keys, player, ResultCode}, cursor, structs::{Bullet, Player}, utils::Utils};
use bevy::{
    app::Startup, asset::AssetServer, ecs::{
        event::EventReader, query::With, system::{Commands, Query, Res}
    }, input::{keyboard::KeyCode, Input}, math::Quat, prelude::{App, Plugin, Update, Vec2, Vec3, Mut}, render::camera::Camera, sprite::SpriteBundle, time::Time, transform::components::{GlobalTransform, Transform}, utils::default, window::{CursorMoved, PrimaryWindow, Window}
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
    mut players: Query<(&mut Transform, &mut Player), With<Player>>,
) {

    let player_tuple = players.single_mut();
    
    let translation = find_player_translation(&input, &time);
    let Ok(translation) = translation else {
        return
    };
    
    apply_player_translation(translation, player_tuple);
}

fn find_player_translation(input: &Res<Input<KeyCode>>, time: &Res<Time<>>) -> Result<Vec2, ResultCode> {

    let speed = find_speed(input);
    let mut translation = Vec2::new(0., 0.);

    // Replace this with a match expression later

    if input.pressed(control_keys::MOVE_UP) {
        translation.y += speed;
    }

    if input.pressed(control_keys::MOVE_DOWN) {
        translation.y -= speed;
    }

    if input.pressed(control_keys::MOVE_LEFT) {
        translation.x -= speed;
    }

    if input.pressed(control_keys::MOVE_RIGHT) {
        translation.x += speed;
    }

    if translation.x == 0. && translation.y == 0. {
        return Err(ResultCode::NoAction)
    };

    let delta_seconds = time.delta_seconds();
    translation.x *= delta_seconds;
    translation.y *= delta_seconds;

    return Ok(translation)
}

fn find_speed(input: &Res<Input<KeyCode>>) -> f32 {
    if input.pressed(control_keys::BOOST) {
        return player::BOOST_SPEED as f32
    }

    // Otherwise we aren't boosting

    return player::SPEED as f32
}

fn apply_player_translation(translation: Vec2, player_tuple: (Mut<'_, Transform>, Mut<'_, Player>)) {
    let mut player_transform = player_tuple.0;
    let player_direction = player_transform.local_y().truncate();
    let translation_direction = translation + player_transform.translation.truncate();

    //let angle; 
    let to_translation = translation + player_transform.translation.truncate();

    /* let angle = player_direction.angle_between(translation_direction); */
    /* let rotation = Quat::from_rotation_arc(Vec3::Y, to_translation.extend(0.)); */
    let angle = /* to_translation.angle_between(player_transform.translation.truncate()) */Utils::find_angle(to_translation.x, to_translation.y, player_transform.translation.x, player_transform.translation.y);
    let rotation = Quat::from_rotation_z(angle);
    println!("Change player angle {}, rotation {}" , angle, rotation);
    player_transform.rotation = rotation;

    player_transform.translation.x += translation.x;
    player_transform.translation.y += translation.y;
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

/*     let cursor_pos = find_cursor_position(cameras, windows);
    let player_tuple = player_positions.single();
    let player_pos = player_tuple.0.translation.truncate(); */
/* 
    let player_cursor_angle = Utils::find_angle(cursor_pos.x, cursor_pos.y, player_pos.x, player_pos.y) + (PI/2.);
    println!("angle: {}, w: {}, z: {}", player_cursor_angle, Quat::from_axis_angle(Vec3::new(0., 0., 1.), player_cursor_angle).w, Quat::from_axis_angle(Vec3::new(0., 0., 1.), player_cursor_angle).z);
 */
/*     let player_test_pos = player_tuple.0.local_y().truncate();
    let test = player_test_pos.angle_between(cursor_pos - player_test_pos);
    println!("alt angle: {}", test); */

    // Considering all these tags to have one unique entity each

    let (camera, camera_transform) = cameras.single();
    let window = windows.single();

    // Extract cursor position if it is inside window
    let Some(cursor_pos) = window 
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    else {
        return
    };

    let player_transform = player_positions.single().0;
    // In 2d up is positive y, so current forward direction for the player
    // is local_y unit vector
    let player_dir = player_transform.local_y().truncate();

    // Direction to cursor (what we want local_y to become) is simply the
    // difference of target position and player position
    let cursor_dir = cursor_pos - player_transform.translation.truncate();
    
    // Then we find the angle between current forward direction and desired one
    let angle = player_dir.angle_between(cursor_dir);
    let rotation = Quat::from_rotation_z(angle);
    // println!("angle {}, x {}, y {}, z {}, w {}", angle, rotation.x, rotation.y, rotation.z, rotation.w);

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
                    rotation,
                    ..default()
                },
                ..default()
            },
            Bullet, 
        ));
    }

/*     // The player is shooting

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
                    rotation: /* Quat::from_axis_angle(Vec3::new(0., 0., 1.), player_cursor_angle) */Quat::from_rotation_z(test),
                    ..default()
                },
                ..default()
            },
            Bullet, 
        ));
    } */
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