use bevy::{app::{Plugin, App, Update}, ecs::{system::{Res, Query}, query::With}, time::Time, transform::components::Transform};

use crate::structs::{Player, Bullet};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_bullets);
    }
}

fn move_bullets(time: Res<Time>, mut bullet_positions: Query<(&mut Transform), With<Bullet>>) {

    for (mut transform) in &mut bullet_positions {
        transform.translation.y += 300. * time.delta_seconds();
    }
}