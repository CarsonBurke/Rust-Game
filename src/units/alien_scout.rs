use std::f32::consts::PI;

use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::AssetServer,
    ecs::{
        entity::Entity,
        query::{With, Without},
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res},
    },
    math::Vec3,
    sprite::SpriteBundle,
    time::Time,
    transform::components::Transform,
    utils::default,
};

use crate::{
    constants::alien_scout,
    structs::{Alien, AlienScout, AlienUnit, Gun, Player},
};

use super::try_fire_all_guns;

pub struct AlienScoutPlugin;

impl Plugin for AlienScoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (kill_aliens_without_health, aliens_shoot).chain());
    }
}

fn kill_aliens_without_health(
    mut aliens: Query<(&mut AlienScout, Entity, &mut Transform)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut alien, entity, transform) in &mut aliens {
        // alien.health -= 1. * time.delta_seconds();

        if alien.health > 0. {
            continue;
        }

        println!("enemy unit is out of health");
        commands.entity(entity).despawn();
    }
}

fn aliens_shoot(
    mut aliens: Query<(&mut AlienScout, &mut Transform), (With<AlienScout>, Without<Player>)>,
    mut players: Query<(&mut Player, &mut Transform), (With<Player>, Without<AlienScout>)>,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let player_tuple = players.single();

    for (mut alien, mut alien_transform) in &mut aliens {
        try_fire_all_guns(
            &mut alien.guns,
            &mut alien_transform,
            &player_tuple.1.translation.truncate(),
            &time,
            &mut commands,
            &asset_server,
        )
    }
}
