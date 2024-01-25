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
}

pub mod player {
    pub const ASSET_PATH: &str = "player.png";
    pub const SPEED: i32 = 150;
    pub const BOOST_SPEED: i32 = 220;
    pub const WIDTH: i32 = 98;
    pub const HEIGHT: i32 = 75;
}

pub mod bullet {
    pub const WIDTH: i32 = 9;
    pub const HEIGHT: i32 = 54;
}

pub mod bullet_player {
    pub const ASSET_PATH: &str = "player_laser.png";
}