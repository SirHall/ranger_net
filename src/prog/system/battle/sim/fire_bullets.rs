use bevy::prelude::*;
use bevy_ggrs::PlayerInputs;

use crate::prog::{
    component::player::Player,
    resource::assets::ProgAssets,
    system::{inputs::INPUT_PRIMARY, matchbox::Config},
};

pub fn fire_bullets(
    mut commands: Commands,
    inputs: Res<PlayerInputs<Config>>,
    images: Res<ProgAssets>,
    players: Query<(&Transform, &Player)>,
) {
    for (transform, player) in &players {
        let (input, _) = inputs[player.handle];

        // Fire bullet
        if input & INPUT_PRIMARY != 0 {
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
