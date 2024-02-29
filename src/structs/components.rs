use bevy::{math::Vec2, prelude::Component};

use super::Gun;

#[derive(Component)]
pub struct Player {
    pub health: f32,
    pub acceleration_x: f32,
    pub acceleration_y: f32,
    pub guns: Vec<Gun>,
}

#[derive(Component)]
pub struct PlayerScout {

}

#[derive(Component)]
pub struct Alien {}

#[derive(Component)]
pub struct AlienUnit {
    
}

#[derive(Component)]
pub struct AlienScout {
    pub health: f32,
    pub guns: Vec<Gun>,
}

#[derive(Component)]
pub struct Unit {
    pub guns: Vec<Gun>,
}

#[derive(Component)]
pub struct Bullet {
    pub time_created: f32,
    pub speed: f32,
    pub range: f32,
}

#[derive(Component, Clone)]
pub struct LightTile;

#[derive(Component)]
pub struct TerrainTile;

#[derive(Component)]
pub struct CameraShake {
    strength: f32,
    /**
     * How many ticks until the screen shake will stop
     */
    remaining_ticks: i32,
}

#[derive(Component)]
pub struct UnitFactory {
    pub last_produced: f32,
}

#[derive(Component)]
pub struct ScoutFactory {

}

pub struct PathfinderGoal {
    pub pos: Vec2,
    pub range: f32,
}