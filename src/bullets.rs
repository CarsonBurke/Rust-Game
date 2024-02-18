use std::f32::consts::PI;

use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, Query, Res},
    },
    math::Vec3,
    time::Time,
    transform::components::Transform,
};

use crate::{
    constants::bullet,
    structs::{Bullet, Player},
};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_bullets, delete_old_bullets));
    }
}

fn delete_old_bullets(
    time: Res<Time>,
    mut bullets: Query<(Entity, &mut Bullet)>,
    mut commands: Commands,
) {
    for (entity, bullet) in &mut bullets {
        let current_time = time.elapsed().as_secs_f32();
        let time_since_created = current_time - bullet.time_created;
        let bullet_age_as_distance: f32 = time_since_created * bullet.speed;

        if bullet_age_as_distance > bullet.range {
            // println!("too old {} {}", bullet.time_created, bullet_age_as_distance);
            commands.entity(entity).despawn();
        }
    }
}

fn move_bullets(time: Res<Time>, mut bullets: Query<(&mut Transform, &mut Bullet), With<Bullet>>) {
    for (mut transform, bullet) in &mut bullets {
        let angle = transform.rotation.w;

        let direction = transform.rotation * Vec3::Y;
        let translation_delta = direction * bullet.speed * time.delta_seconds();

        transform.translation += translation_delta;
        /* println!("z: {}, w: {}", transform.rotation.z * PI, transform.rotation.w * PI); */
    }
}
