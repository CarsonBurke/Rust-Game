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
}