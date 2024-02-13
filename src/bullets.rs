use std::f32::consts::PI;

use bevy::{app::{App, Plugin, Update}, ecs::{query::With, system::{Query, Res}}, math::Vec3, time::Time, transform::components::Transform};

use crate::structs::{Player, Bullet};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_bullets);
    }
}

fn move_bullets(time: Res<Time>, mut bullet_positions: Query<(&mut Transform), With<Bullet>>) {

    let speed: f32 = 300.;

    for (mut transform) in &mut bullet_positions {
        
        let angle = transform.rotation.w;
/*         let x_diff = (speed * (angle.abs()).cos())/* .abs() */;
        let y_diff = (speed * (angle.abs()).sin())/* .abs() */; */
/*         let x_diff = speed * (angle).cos()/* .abs() */;
        let y_diff = speed * (angle).sin()/* .abs() */; */
/*         let x_diff = transform.rotation.w * speed;
        let y_diff = -transform.rotation.z * speed;

        transform.translation.x += x_diff * time.delta_seconds();
        transform.translation.y += y_diff * time.delta_seconds(); */

        let direction = transform.rotation * Vec3::Y;
        let translation_delta = direction * speed * time.delta_seconds();

        transform.translation += translation_delta;
        /* println!("z: {}, w: {}", transform.rotation.z * PI, transform.rotation.w * PI); */
    }
}