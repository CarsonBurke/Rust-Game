use crate::types::Direction;
use bevy::{
    asset::{AssetServer, Handle},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::system::{Commands, Query, Res, ResMut},
    render::{texture::Image, view::WindowSurfaces},
    sprite::SpriteBundle,
    transform::components::Transform,
    utils::default,
    window::Window,
};

pub struct GameInit;

const PLAYER_SPRITE: &str = "player.png";
struct GameTextures {
    player: Handle<Image>,
}

pub fn game_init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(PLAYER_SPRITE),
            transform: Transform::from_xyz(100., 100., 0.),
            ..default()
        },
        Direction::Up,
    ));

    /* let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE)
    };
    commands.insert_resource(game_textures); */
}
