pub mod battle;
pub mod inputs;
pub mod matchbox;

use bevy::{prelude::*, render::camera::ScalingMode};

pub fn init(mut commands: Commands) {
    // Spawn camera
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(10.);
    commands.spawn(camera_bundle);
}
