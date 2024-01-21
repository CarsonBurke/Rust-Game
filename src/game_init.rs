use bevy::{ecs::system::{Commands, Res, Query, ResMut}, core_pipeline::core_2d::Camera2dBundle, sprite::SpriteBundle, asset::{AssetServer, Handle}, utils::default, render::{texture::Image, view::WindowSurfaces}, window::Window, transform::components::Transform};

pub struct GameInit;

const PLAYER_SPRITE: &str = "player.png";
struct GameTextures {
    player: Handle<Image>,
}

pub fn game_init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load(PLAYER_SPRITE),
        transform: Transform::from_xyz(100., 100., 0.),
        ..default()
    });

    /* let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE)
    };
    commands.insert_resource(game_textures); */
}