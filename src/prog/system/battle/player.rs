use crate::prog::{
    component::player::Player,
    system::{
        inputs::{INPUT_DOWN, INPUT_LEFT, INPUT_RIGHT, INPUT_UP},
        matchbox::Config,
    },
};
use bevy::prelude::*;
use bevy_ggrs::{AddRollbackCommandExtension, PlayerInputs};

pub fn spawn_players(mut commands: Commands) {
    // Player 1
    commands
        .spawn((
            Player { handle: 0 },
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(-2., 0., 1.)),
                sprite: Sprite {
                    color: Color::srgb(0., 0.47, 1.),
                    custom_size: Some(Vec2::new(1., 1.)),
                    ..default()
                },
                ..default()
            },
        ))
        .add_rollback();

    // Player 2
    commands
        .spawn((
            Player { handle: 1 },
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(2., 0., 1.)),
                sprite: Sprite {
                    color: Color::srgb(0., 0.4, 0.),
                    custom_size: Some(Vec2::new(1., 1.)),
                    ..default()
                },
                ..default()
            },
        ))
        .add_rollback();
}

pub fn move_players(mut players: Query<(&mut Transform, &Player)>, inputs: Res<PlayerInputs<Config>>, time: Res<Time>) {
    for (mut transform, player) in &mut players {
        // println!("INPUT: {:?} {:?}", player.handle, inputs[player.handle].0);
        let (input, _) = inputs[player.handle];

        let mut direction = Vec2::ZERO;

        if input & INPUT_UP != 0 {
            direction.y += 1.;
        }
        if input & INPUT_DOWN != 0 {
            direction.y -= 1.;
        }
        if input & INPUT_RIGHT != 0 {
            direction.x += 1.;
        }
        if input & INPUT_LEFT != 0 {
            direction.x -= 1.;
        }
        if direction == Vec2::ZERO {
            continue;
        }

        let move_speed = 7.;
        let move_delta = direction * move_speed * time.delta_seconds();
        transform.translation += move_delta.extend(0.);
    }
}
