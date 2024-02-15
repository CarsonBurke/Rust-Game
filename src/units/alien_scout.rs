use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::AssetServer,
    ecs::system::{Commands, Query, Res},
    math::Vec3,
    sprite::SpriteBundle,
    time::Time,
    transform::components::Transform,
    utils::default,
};

use crate::{
    constants::alien_scout,
    structs::{Alien, AlienScout, AlienUnit},
};

pub struct AlienScoutPlugin;

impl Plugin for AlienScoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_aliens)
            .add_systems(Update, (run_alien_scouts));
    }
}

fn spawn_aliens(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(alien_scout::ASSET_PATH),
            transform: Transform {
                translation: Vec3::new(200., 200., alien_scout::Z_POS),
                ..default()
            },
            ..default()
        },
        AlienScout { health: 100. },
    ));
}

fn run_alien_scouts(mut aliens: Query<(&mut AlienScout, &mut Transform)>, time: Res<Time>) {
    for (mut alien, transform) in &mut aliens {
        alien.health -= 1. * time.delta_seconds();
        if alien.health <= 0. {
            println!("enemy unit is out of health");
            continue;
        }
    }
}
