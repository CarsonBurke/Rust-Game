use std::f32::consts::PI;

use bevy::{
    asset::AssetServer,
    ecs::{
        system::{Commands, Res},
        world::Mut,
    },
    math::{Quat, Vec2, Vec3},
    sprite::SpriteBundle,
    time::Time,
    transform::components::Transform,
    utils::default,
};

use crate::{
    constants::bullet,
    structs::{Bullet, Gun, Unit},
    utils,
};

pub fn try_fire_all_guns(
    mut guns: &mut Vec<Gun>,
    mut unit_transform: &mut Mut<Transform>,
    mut target_pos: &Vec2,
    time: &Res<Time>,
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {

    let max_range = find_max_range_of_guns(guns);
    let range_to_target: f32 = utils::range(
        unit_transform.translation.x,
        unit_transform.translation.y,
        target_pos.x,
        target_pos.y,
    );
    if range_to_target > max_range * 1.1 {
        return
    }

    // Then we find the angle between current forward direction and desired one
    let angle = utils::find_angle(unit_transform.translation.x, unit_transform.translation.y, target_pos.x, target_pos.y) - PI/2./* unit_dir.angle_between(cursor_dir) */;
    let rotation = Quat::from_rotation_z(angle);

    unit_transform.rotation = rotation;

    for mut gun in guns {

        if is_gun_out_of_range(gun, unit_transform, target_pos) {
            continue
        }

        if is_gun_on_cooldown(gun, time) {
            continue
        }

        let bullet_angle = angle + rand::random::<f32>() * gun.arc - rand::random::<f32>() * gun.arc;
        let bullet_rotation = Quat::from_rotation_z(bullet_angle);

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(gun.asset_path.clone()),
                transform: Transform {
                    translation: Vec3 {
                        x: unit_transform.translation.x,
                        y: unit_transform.translation.y,
                        z: bullet::Z_POS,
                    },
                    rotation: bullet_rotation,
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

fn find_max_range_of_guns(guns: &mut Vec<Gun>) -> f32 {
    let mut max_range: f32 = 0.;
    for gun in guns {
        if gun.range <= max_range {
            continue;
        };

        max_range = gun.range
    }

    max_range
}

fn is_gun_out_of_range(gun: &mut Gun, unit_transform: &mut Transform, target_pos: &Vec2) -> bool {
    let range = utils::range(
        unit_transform.translation.x,
        unit_transform.translation.y,
        target_pos.x,
        target_pos.y,
    );
    if range > gun.range {
        return true;
    }

    false
}

fn is_gun_on_cooldown(gun: &mut Gun, time: &Res<Time>) -> bool {
    // with a fire rate of five, the interval will be 1 / 5 = 0.2
    let fire_interval = 1. / gun.fire_rate;
    let time_since_last_shot = time.elapsed_seconds() - gun.last_shot;

    // If we have shot more recently than our fire rate allows, don't allow the gun to shoot
    if time_since_last_shot < fire_interval {
        return true;
    }

    false
}
