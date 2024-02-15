pub mod map {
    pub const DIMENSIONS: i64 = 10000;
}

pub mod control_keys {
    use bevy::input::keyboard::KeyCode;

    pub const MOVE_UP: KeyCode = KeyCode::W;
    pub const MOVE_DOWN: KeyCode = KeyCode::S;
    pub const MOVE_LEFT: KeyCode = KeyCode::A;
    pub const MOVE_RIGHT: KeyCode = KeyCode::D;
    pub const BOOST: KeyCode = KeyCode::ShiftLeft;
    pub const SHOOT: KeyCode = KeyCode::Space;
}

pub mod tiles {
    pub const SIZE: i32 = 250;
}

pub mod tile_grass {
    pub const ASSET_PATH: &str = "grass.png";
    pub const Z_POS: f32 = 0.;
}

pub mod player {
    pub const ASSET_PATH: &str = "player.png";
    pub const SPEED: f32 = 150.;
    pub const BOOST_SPEED: f32 = 220.;
    pub const WIDTH: i32 = 98;
    pub const HEIGHT: i32 = 75;
    pub const Z_POS: f32 = 1000.;
}

pub mod alien_unit {

}

pub mod alien_scout {
    pub const ASSET_PATH: &str = "player.png";
    pub const Z_POS: f32 = 1000.;
    pub const SPEED: f32 = 100.;
}

pub mod bullet {
    pub const WIDTH: i32 = 9;
    pub const HEIGHT: i32 = 54;
    pub const Z_POS: f32 = 2.;
}

pub mod bullet_player {
    pub const ASSET_PATH: &str = "player_laser.png";
}

pub mod window {
    pub const ASSET_PATH: &str = "assets/player.png";
    pub const PRIMARY_WINDOW_TITLE: &str = "Learning Bevy Game";
}

pub enum ResultCode {
    Success,
    Failure,
    NoAction,
}