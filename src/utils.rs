use bevy::math::{Vec2, Vec3};

pub struct Utils;

impl Utils {
    pub fn range(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
        return 0.1;
    }

    pub fn find_angle(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
        
        let x_diff = (x2 - x2).abs();
        let y_diff = (y1 - y2).abs();

        let angle = x_diff.atan2(y_diff);
        return angle;
    }
}