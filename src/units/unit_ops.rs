use std::f32::consts::PI;

use bevy::{
    asset::AssetServer,
    ecs::{system::{Commands, Res}, world::Mut},
    math::{Quat, Vec3},
    sprite::SpriteBundle,
    time::Time,
    transform::components::Transform,
    utils::default,
};

use crate::{
    constants::bullet,
    structs::{Bullet, Gun, Unit},
    utils::Utils,
};

pub fn try_fire_all_guns(
    mut guns: &mut Vec<Gun>,
    mut unit_transform: &mut Mut<Transform>,
    mut target_transform: &Transform,
    time: &Res<Time>,
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    // Then we find the angle between current forward direction and desired one
    let angle = Utils::find_angle(unit_transform.translation.x, unit_transform.translation.y, target_transform.translation.x, target_transform.translation.y) - PI/2./* unit_dir.angle_between(cursor_dir) */;
    let rotation = Quat::from_rotation_z(angle);

    unit_transform.rotation = rotation;

    for mut gun in guns {
        // with a fire rate of five, the interval will be 1 / 5 = 0.2
        let fire_interval = 1. / gun.fire_rate;
        let time_since_last_shot = time.elapsed_seconds() - gun.last_shot;

        // If we have shot more recently than our fire rate allows, don't allow the gun to shoot
        if time_since_last_shot < fire_interval {
            continue;
        }

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(gun.asset_path.clone()),
                transform: Transform {
                    translation: Vec3 {
                        x: unit_transform.translation.x,
                        y: unit_transform.translation.y,
                        z: bullet::Z_POS,
                    },
                    rotation,
                    ..default()
                },
                ..default()
            },
            Bullet {
                time_created: time.elapsed_seconds(),
                speed: gun.speed,
                range: gun.range,
            },
        ));

        gun.last_shot = time.elapsed_seconds();
    }
}
