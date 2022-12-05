use bevy::{prelude::*, render::camera::ScalingMode};

use crate::TILE_SIZE;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    // camera.projection.top = 1.0;
    // camera.projection.bottom = -1.0;
    camera.projection.top = TILE_SIZE * 4.5;
    camera.projection.bottom = -TILE_SIZE * 4.5;

    // camera.projection.right = 1.0 * RESOLUTION;
    // camera.projection.left = -1.0 * RESOLUTION;
    camera.projection.right = TILE_SIZE * 8.0;
    camera.projection.left = -TILE_SIZE * 8.0;

    camera.projection.scaling_mode = ScalingMode::None;

    commands.spawn(camera);
}
