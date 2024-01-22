use bevy::prelude::Component;

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Bullet;