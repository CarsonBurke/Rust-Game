use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::AssetServer,
    ecs::system::{Commands, Res},
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

pub struct UnitFactoryPlugin;

impl Plugin for UnitFactoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_unit_factory)
            .add_systems(Update, (create_unit));
    }
}

fn spawn_unit_factory(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(scout_factory::ASSET_PATH),
            transform: Transform {
                translation: Vec3::new(400., 400., scout_factory::Z_POS),
                ..default()
            },
            ..default()
        },
        UnitFactory { last_produced: 0. },
        ScoutFactory {},
    ));
}

fn create_unit(time: Res<Time>) {}
