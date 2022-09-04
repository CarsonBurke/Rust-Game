#[allow(dead_code)]

use bevy::prelude::*;

// region: Asset Constants

const PLAYER_SPRITE: &str = "player.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);

const SPRITE_SCALE: f32 = 0.5;

// region: Resources

pub struct WindowSize {
    pub w: f32,
    pub h: f32,
}

struct GameTextures {
    player: Handle<Image>,
}

// endregion: Resources

// endregion: Asset Constants

fn main() {
    App::new()
    .insert_non_send_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .insert_resource(WindowDescriptor {
        title: "Rust Invaders".to_string(),
        width: 598.,
        height: 676.,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup_system)
    .add_startup_system_to_stage(StartupStage::PostStartup, create_player_system)
    .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>
) {
    
    commands.spawn_bundle(Camera2dBundle::default());

    // Get window size

    let window = windows.get_primary_mut().unwrap();
    let window_width = window.width();
    let window_height = window.height();

    window.set_position(IVec2::new(2780, 4900));

    // Establish structures

    let window_size = WindowSize { w: window_width, h: window_height };
    commands.insert_resource(window_size);

    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE)
    };
    commands.insert_resource(game_textures);

}

fn create_player_system (
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    window_size: Res<WindowSize>
) {

    let bottom = -window_size.h / 2.;

    commands.spawn_bundle(SpriteBundle {
        texture: game_textures.player.clone(),
        transform: Transform {
            translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5., 10.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}