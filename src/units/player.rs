use std::f32::consts::PI;

use crate::{
    constants::{bullet, control_keys, player, ResultCode},
    cursor,
    structs::{Bullet, Gun, Player},
    utils::Utils,
};
use bevy::{
    app::Startup,
    asset::AssetServer,
    ecs::{
        event::EventReader,
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res},
    },
    input::{keyboard::KeyCode, Input},
    math::Quat,
    prelude::{App, Mut, Plugin, Update, Vec2, Vec3},
    render::camera::Camera,
    sprite::SpriteBundle,
    time::Time,
    transform::components::{GlobalTransform, Transform},
    utils::default,
    window::{CursorMoved, PrimaryWindow, Window},
};

use super::try_fire_all_guns;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            (control_player_movement, control_player_shooting).chain(),
        );
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(player::ASSET_PATH),
            transform: Transform {
                translation: Vec3::new(100., 100., player::Z_POS),
                ..default()
            },
            ..default()
        },
        Player {
            health: 100.,
            acceleration_x: 0.,
            acceleration_y: 0.,
            guns: vec![Gun {
                range: 600.,
                speed: 600.,
                fire_rate: 5.,
                last_shot: 0.,
                asset_path: String::from("player_laser.png"),
                arc: PI / 16.,
            }],
        },
    ));
}

fn control_player_movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut players: Query<(&mut Transform, &mut Player), With<Player>>,
) {
    let player_tuple = players.single_mut();

    let translation = find_player_translation(&input, &time);
    let Ok(translation) = translation else { return };

    apply_player_translation(translation, player_tuple);
}

fn find_player_translation(
    input: &Res<Input<KeyCode>>,
    time: &Res<Time>,
) -> Result<Vec2, ResultCode> {
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
        return Err(ResultCode::NoAction);
    };

    let delta_seconds = time.delta_seconds();
    translation.x *= delta_seconds;
    translation.y *= delta_seconds;

    return Ok(translation);
}

fn find_speed(input: &Res<Input<KeyCode>>) -> f32 {
    if input.pressed(control_keys::BOOST) {
        return player::BOOST_SPEED as f32;
    }

    // Otherwise we aren't boosting

    return player::SPEED as f32;
}

fn apply_player_translation(translation: Vec2, player_tuple: (Mut<Transform>, Mut<Player>)) {
    let mut player_transform = player_tuple.0;
    let player_direction = player_transform.local_y().truncate();
    let translation_direction = translation + player_transform.translation.truncate();

    //let angle;
    let to_translation = translation + player_transform.translation.truncate();

    /* let angle = player_direction.angle_between(translation_direction); */
    /* let rotation = Quat::from_rotation_arc(Vec3::Y, to_translation.extend(0.)); */
    let angle = /* player_transform.local_y().truncate().angle_between(translation * player_transform.translation.truncate()) */Utils::find_angle( to_translation.x, to_translation.y, player_transform.translation.x, player_transform.translation.y) + PI/2.;
    let rotation = Quat::from_rotation_z(angle);
    /* println!("Change player angle {}, rotation {}" , angle, rotation); */
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
    mut players: Query<(&mut Transform, &mut Player), With<Player>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    if !input.pressed(control_keys::SHOOT) {
        return;
    }

    // Considering all these tags to have one unique entity each

    let (camera, camera_transform) = cameras.single();
    let window = windows.single();

    // Extract cursor position if it is inside window
    let Some(cursor_pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    else {
        return;
    };

    // println!("angle {}, x {}, y {}, z {}, w {}", angle, rotation.x, rotation.y, rotation.z, rotation.w);

    // The player is shooting

    for (mut player_transform, mut player) in &mut players {
        try_fire_all_guns(
            &mut player.guns,
            &mut player_transform,
            &cursor_pos,
            &time,
            &mut commands,
            &asset_server,
        );

        // Then we find the angle between current forward direction and desired one
        let angle = Utils::find_angle(player_transform.translation.x, player_transform.translation.y, cursor_pos.x, cursor_pos.y) - PI/2./* player_dir.angle_between(cursor_dir) */;
        let rotation = Quat::from_rotation_z(angle);

        player_transform.rotation = rotation;

        // with a fire rate of five, the interval will be 1 / 5 = 0.2
        let fire_interval = 1. / player.guns[0].fire_rate;
        let time_since_last_shot = time.elapsed_seconds() - player.guns[0].last_shot;

        // If we have shot more recently than our fire rate allows, don't allow the gun to shoot
        if time_since_last_shot < fire_interval {
            continue;
        }

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(player.guns[0].asset_path.clone()),
                transform: Transform {
                    translation: Vec3 {
                        x: player_transform.translation.x,
                        y: player_transform.translation.y,
                        z: bullet::Z_POS,
                    },
                    rotation,
                    ..default()
                },
                ..default()
            },
            Bullet {
                time_created: time.elapsed_seconds(),
                speed: player.guns[0].speed,
                range: player.guns[0].range,
            },
        ));

        player.guns[0].last_shot = time.elapsed_seconds();
    }
}

fn players_shoot(mut player_positions: &mut Query<(&mut Transform), With<Player>>) {}

fn find_cursor_position(
    cameras: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window, With<PrimaryWindow>>,
) -> Vec2 {
    let (camera, camera_transform) = cameras.single();

    let Some(cursor_absolute_pos) = windows.single().cursor_position() else {
        return Vec2 { x: 0., y: 0. };
    };

    // Calculate a world position based on the cursor's position.
    let Some(cursor_pos) = camera.viewport_to_world_2d(camera_transform, cursor_absolute_pos)
    else {
        return Vec2 { x: 0., y: 0. };
    };

    return cursor_pos;
}

fn cursor_events(mut cursor_evr: EventReader<CursorMoved>) {
    for ev in cursor_evr.read() {
        println!(
            "New cursor position: X: {}, Y: {}, in Window ID: {:?}",
            ev.position.x, ev.position.y, ev.window
        );
    }
}
