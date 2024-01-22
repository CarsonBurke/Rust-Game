use crate::{structs::{Bullet, Player}, constants::control_keys};
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
    transform::components::{Transform, GlobalTransform},
    utils::default, render::camera::Camera,
};

pub struct CameraControlsPlugin;

impl Plugin for CameraControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (control_camera_viewport));
    }
}

fn control_camera_viewport(input: Res<Input<KeyCode>>, mut camera_positions: Query<&mut Transform, With<Camera>>, time: Res<Time>) {

    if input.pressed(control_keys::MOVE_UP) {
        move_camera_viewport(&mut camera_positions, &time, 0., 100.);
    }

    if input.pressed(control_keys::MOVE_DOWN) {
        move_camera_viewport(&mut camera_positions, &time, 0., -100.);
    }

    if input.pressed(control_keys::MOVE_LEFT) {
        move_camera_viewport(&mut camera_positions, &time, -100., 0.);
    }

    if input.pressed(control_keys::MOVE_RIGHT) {
        move_camera_viewport(&mut camera_positions, &time, 100., 0.);
    }
}

fn move_camera_viewport(
    mut camera_positions: &mut Query<(&mut Transform), With<Camera>>,
    time: &Res<Time>,
    x: f32,
    y: f32,
) {
    for (mut transform) in camera_positions {
        transform.translation.x += x * time.delta_seconds();
        transform.translation.y += y * time.delta_seconds();
    }
}