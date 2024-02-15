use bevy::prelude::Component;

#[derive(Component)]
pub struct Player {
    pub health: f32,
    pub acceleration_x: f32,
    pub acceleration_y: f32,
}

#[derive(Component)]
pub struct Alien {}

#[derive(Component)]
pub struct AlienUnit {
    pub health: f32,
}

#[derive(Component)]
pub struct AlienScout {
    
}

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct CameraShake {
    strength: f32,
    /**
     * How many ticks until the screen shake will stop
     */
    remaining_ticks: i32,
}
