use bevy::app::{App, Plugin, Update};

use crate::{constants::{light_tiles, LandImpassibleTerrainTypes}, structs::{LightTile, TerrainTile}};

pub struct LightsPlugin;

impl Plugin for LightsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_lights,));
    }
}

fn update_lights() {
    let terrain_tiles: Vec<TerrainTile> = Vec::new();
    let mut light_tiles: Vec<TerrainTile> = Vec::new();
    let visited_tiles: Vec<TerrainTile> = Vec::new();
    let mut current_tiles: Vec<LightTile> = Vec::new();

    while current_tiles.len() > 0 {

        let mut next_tiles: Vec<LightTile> = Vec::new();

        for tile in current_tiles {
            let terrain_tile = tile.get_terrain();

            if (LandImpassibleTerrainTypes.has(terrain_tile)) {
                continue;
            }


        }

        current_tiles = next_tiles.clone();
        next_tiles = vec![]
    }
}

fn light_noise() {
    let light_tiles: Vec<LightTile> = Vec::new();

    for tile in light_tiles {
        tile.brightness -= rand::random() * 5;
    }
}