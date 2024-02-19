use std::f32::consts::PI;

use bevy::math::{Vec2, Vec3};

pub struct Utils;

impl Utils {
    pub fn range(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
        return ((x1 - x2).abs()).max((y1 - y2).abs());
    }

    pub fn find_angle(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
        /*         let x_diff = (x1 - x2).abs();
        let y_diff = (y1 - y2).abs(); */
        let x_diff = /* (x2 - x1).abs() */(x2 - x1);
        let y_diff = /* (y2 - y1).abs() */(y2 - y1);

        let mut angle = y_diff.atan2(x_diff);
        /*         if angle < 0. {
            angle = PI + angle.abs();
        }; */
        return angle;
    }
}
