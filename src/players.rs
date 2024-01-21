use bevy::{ecs::system::{Res, Query}, time::Time, utils::petgraph::Direction, transform::components::Transform};

pub fn player_actions(time: Res<Time>, mut sprite_pos: Query<(&mut Transform)>) {
    for (mut transform) in &mut sprite_pos {
        transform.translation.x += 100. * time.delta_seconds();
        transform.translation.y += 100. * time.delta_seconds();
    }
}