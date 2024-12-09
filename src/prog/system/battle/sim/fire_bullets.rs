use crate::prog::{
    component::player::Player,
    resource::assets::ProgAssets,
    system::{
        inputs::{CAMERA_CRUSH_FACTOR, INPUT_LOOK_X, INPUT_LOOK_Y, INPUT_PRIMARY},
        matchbox::Config,
    },
};
use bevy::prelude::*;
use bevy_ggrs::PlayerInputs;
use bytemuck::cast;

pub fn fire_bullets(
    mut commands: Commands,
    inputs: Res<PlayerInputs<Config>>,
    images: Res<ProgAssets>,
    players: Query<(&Transform, &Player)>,
) {
    for (transform, player) in &players {
        let (input, _) = inputs[player.handle];

        let dx = (cast::<u8, i8>(((input & INPUT_LOOK_X) >> 8) as u8) as f32) / CAMERA_CRUSH_FACTOR;
        let dy = (cast::<u8, i8>(((input & INPUT_LOOK_Y) >> 16) as u8) as f32) / CAMERA_CRUSH_FACTOR;

        // let dir = Vec2::new(dx, dy).normalize_or_zero();
        // let diff = Vec2::new(dx, dy);

        // Fire bullet
        if input & INPUT_PRIMARY != 0 {
            let dir = Vec2::new(dx, dy).normalize_or_zero();
            commands.spawn(SpriteBundle {
                transform: Transform::from_translation(transform.translation),
                texture: images.bullet.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(0.3, 0.1)),
                    ..default()
                },
                ..default()
            });
        }
    }
}
