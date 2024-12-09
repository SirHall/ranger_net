use crate::prog::{
    component::player::Player,
    system::{
        inputs::{INPUT_DOWN, INPUT_LEFT, INPUT_RIGHT, INPUT_UP},
        matchbox::Config,
    },
};
use bevy::prelude::*;
use bevy_ggrs::{AddRollbackCommandExtension, PlayerInputs, Session};

pub fn spawn_players(mut commands: Commands, session: Res<Session<Config>>) {
    let num_players = match &*session {
        Session::SyncTest(s) => s.num_players(),
        Session::P2P(s) => s.num_players(),
        Session::Spectator(s) => s.num_players(),
    };

    for i in 0..num_players {
        commands
            .spawn((
                Player { handle: i },
                SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(0. + (i as f32), 0., 1.)),
                    sprite: Sprite {
                        color: Color::srgb(0., 0.47, 1.),
                        custom_size: Some(Vec2::new(1., 1.)),
                        ..default()
                    },
                    ..default()
                },
            ))
            .add_rollback();
    }
}
