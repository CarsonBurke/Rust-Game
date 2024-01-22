use bevy::{app::{App, Plugin, Startup, DynEq}, ecs::system::{Commands, Res}, asset::AssetServer, sprite::SpriteBundle, transform::components::Transform, math::Vec3, utils::default};

use crate::{structs::Tile, constants::{asset_paths, map, tiles}};

pub struct TilesPlugin;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_tiles);
    }
}

fn init_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {

    let mut x: i32 = 0;
    while (x < map::DIMENSIONS as i32) {
        x += tiles::TILE_SIZE;

        let mut y: i32 = 0;

        while (y < map::DIMENSIONS as i32) {
            y += tiles::TILE_SIZE;

            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load(asset_paths::GRASS),
                    transform: Transform {
                        translation: Vec3::new(x as f32, y as f32, 0.),
                        ..default()
                    },
                    ..default()
                },
                Tile,
            ));
        }
    }
}