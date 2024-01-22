pub mod asset_paths {
    pub const PLAYER: &str = "player.png";
    pub const BULLET: &str = "player_laser.png";
    pub const GRASS: &str = "grass.png";
}

pub mod tiles {
    pub const TILE_SIZE: i32 = 250;
}

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

pub mod player {
    pub const SPEED: i32 = 150;
    pub const BOOST_SPEED: i32 = 220;
}