use bevy::utils::{HashMap, HashSet};

pub struct Gun {
    pub range: f32,
    // pub gun_type: str,
    pub speed: f32,
    /**
     * The maximum number of shots per second this gun can output
     */
    pub fire_rate: f32,
    /**
     * The time when the gun most recently shot a bullet
     */
    pub last_shot: f32,
    pub asset_path: String,
    // pub rotation_speed: f32,
    // pub rotation: f32,
    /// the arc, in radians, that the bullet spawned may be in between
    pub arc: f32,
}