use crate::{
    constants::control_keys,
    structs::{Bullet, Player},
};
use bevy::{
    asset::AssetServer,
    ecs::{
        event::EventReader, query::{With, Without}, system::{Commands, Query, Res}
    },
    input::{keyboard::KeyCode, mouse::MouseWheel, Input},
    prelude::{App, Plugin, Update},
    render::camera::{Camera, OrthographicProjection},
    sprite::SpriteBundle,
    time::Time,
    transform::components::{GlobalTransform, Transform},
    utils::default,
};

pub struct CameraControlsPlugin;

impl Plugin for CameraControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (control_camera_viewport, control_camera_zoom));
    }
}

/**
 * follow the player around
 */
fn control_camera_viewport(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Player, (&mut Transform, Without<Camera>))>,
    mut camera_query: Query<(&Camera, (&mut Transform, Without<Player>))>,
    time: Res<Time>,
) {
    let Ok((camera, mut camera_transform)) = camera_query.get_single_mut() else {
        return;
    };

    let Ok((player, mut player_transform)) = player_query.get_single_mut() else {
        return;
    };

    camera_transform.0.translation = player_transform.0.translation.clone();
}

fn control_camera_zoom(mut cameras: Query<&mut OrthographicProjection, With<Camera>>, time: Res<Time>, mut scroll_event_reader: EventReader<MouseWheel>) {
    let mut projection = cameras.single_mut();

    for event in scroll_event_reader.read() {
        let projection_delta = event.y;

        projection.scale -= projection_delta * time.delta_seconds();
    }
}