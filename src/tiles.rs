use bevy::{app::{App, Plugin, Startup, DynEq}, ecs::system::{Commands, Res}, asset::AssetServer, sprite::SpriteBundle, transform::components::Transform, math::Vec3, utils::default};

use crate::{constants::{light_tiles, map, terrain_tiles, tile_grass}, structs::{LightTile, TerrainTile}};

pub struct TilesPlugin;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_tiles);
    }
}

fn init_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {

    let mut x: i32 = 0;
    while (x < map::DIMENSIONS as i32) {
        x += terrain_tiles::SIZE;

        let mut y: i32 = 0;

        while (y < map::DIMENSIONS as i32) {
            y += terrain_tiles::SIZE;

            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load(tile_grass::ASSET_PATH),
                    transform: Transform {
                        translation: Vec3::new(x as f32, y as f32, tile_grass::Z_POS),
                        ..default()
                    },
                    ..default()
                },
                TerrainTile,
            ));
        }
    }
}

fn init_light_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut x: i32 = 0;
    while (x < map::DIMENSIONS as i32) {
        x += light_tiles::SIZE;

        let mut y: i32 = 0;

        while (y < map::DIMENSIONS as i32) {
            y += light_tiles::SIZE;

            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load(tile_grass::ASSET_PATH),
                    transform: Transform {
                        translation: Vec3::new(x as f32, y as f32, tile_grass::Z_POS),
                        ..default()
                    },
                    ..default()
                },
                LightTile,
            ));
        }
    }
}