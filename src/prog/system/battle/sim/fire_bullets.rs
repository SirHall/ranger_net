use crate::prog::{
    component::player::Player,
    resource::assets::ProgAssets,
    system::{
        inputs::{CAMERA_CRUSH_FACTOR, INPUT_LOOK_X, INPUT_LOOK_Y, INPUT_PRIMARY},
        matchbox::Config,
    },
};
use bevy::{color::palettes::css, prelude::*};
use bevy_ggrs::PlayerInputs;
use bevy_mod_raycast::prelude::*;
use bytemuck::cast;

pub fn fire_bullets(
    mut commands: Commands,
    inputs: Res<PlayerInputs<Config>>,
    images: Res<ProgAssets>,
    players: Query<(Entity, &Transform, &Player)>,
    mut raycast: Raycast,
    mut gizmos: Gizmos,
) {
    let fire_dist = 128.0;

    for (player_entity, transform, player) in &players {
        let (input, _) = inputs[player.handle];

        let dx = (cast::<u8, i8>(((input & INPUT_LOOK_X) >> 8) as u8) as f32) / CAMERA_CRUSH_FACTOR;
        let dy = (cast::<u8, i8>(((input & INPUT_LOOK_Y) >> 16) as u8) as f32) / CAMERA_CRUSH_FACTOR;

        // let dir = Vec2::new(dx, dy).normalize_or_zero();
        // let diff = Vec2::new(dx, dy);

        // Fire bullet
        if input & INPUT_PRIMARY != 0 {
            let dir = Vec2::new(dx, dy).normalize_or_zero();
            let dir_length = dir.length_squared();
            if dir_length.is_nan() || dir_length < 0.01 {
                continue;
            }

            gizmos.line_2d(
                transform.translation.truncate(),
                transform.translation.truncate() + (dir * fire_dist),
                css::RED,
            );

            let hits = raycast.debug_cast_ray(
                Ray3d::new(
                    transform.translation,
                    // transform.translation + (dir * fire_dist).extend(1.0),
                    dir.extend(0.0),
                ),
                &RaycastSettings {
                    visibility: RaycastVisibility::MustBeVisibleAndInView,
                    filter: &|hit_entity: Entity| hit_entity != player_entity,
                    ..Default::default()
                },
                &mut gizmos,
            );

            if hits.len() > 0 {
                println!("hits: {:?}", hits);
            }

            // commands.spawn(SpriteBundle {
            //     transform: Transform::from_translation(transform.translation),
            //     texture: images.bullet.clone(),
            //     sprite: Sprite {
            //         custom_size: Some(Vec2::new(0.3, 0.1)),
            //         ..default()
            //     },
            //     ..default()
            // });
        }
    }
}
