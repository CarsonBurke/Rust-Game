use std::f32::consts::PI;

use bevy::{
    app::{App, Plugin, Update},
    asset::AssetServer,
    ecs::system::{Commands, Query, Res},
    math::Vec3,
    sprite::SpriteBundle,
    time::Time,
    transform::components::Transform,
    utils::default,
};

use crate::{
    constants::{alien_scout, scout_factory},
    structs::{AlienScout, Gun, ScoutFactory, UnitFactory},
};

use super::unit_factory;

pub struct ScoutFactoryPlugin;

impl Plugin for ScoutFactoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (create_unit));
    }
}

fn create_unit(
    time: Res<Time>,
    mut scout_factories: Query<(&mut ScoutFactory, &mut UnitFactory, &mut Transform)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (scout_factory, mut factory, factory_transform) in &mut scout_factories {
        let production_interval = scout_factory::PRODUCTION_TIME;
        let time_since_last_produced = time.elapsed_seconds() - factory.last_produced;
        if time_since_last_produced < production_interval {
            continue;
        }

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(alien_scout::ASSET_PATH),
                transform: Transform {
                    translation: Vec3 {
                        x: factory_transform.translation.x + rand::random::<f32>() * 200.,
                        y: factory_transform.translation.y + rand::random::<f32>() * 200.,
                        z: alien_scout::Z_POS,
                    },
                    ..default()
                },
                ..default()
            },
            AlienScout {
                health: 10.,
                guns: vec![Gun {
                    range: 600.,
                    speed: 600.,
                    fire_rate: 2.,
                    asset_path: "alien_laser.png".to_string(),
                    last_shot: 0.,
                    arc: PI / 8.,
                }],
            },
        ));

        factory.last_produced = time.elapsed_seconds();
    }
}
