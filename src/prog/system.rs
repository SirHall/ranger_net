pub mod matchbox;

use bevy::{prelude::*, render::camera::ScalingMode};

pub fn init(mut commands: Commands) {

    // Spawn camera
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(10.);
    commands.spawn(camera_bundle);
    
    // Spawn player
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0., 0.47, 1.),
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        ..default()
    });
}
