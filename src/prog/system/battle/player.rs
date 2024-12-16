use crate::prog::{component::player::Player, system::matchbox::Config};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ggrs::{AddRollbackCommandExtension, Session};

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
                RigidBody::Dynamic,
                Collider::circle(1.0),
            ))
            .add_rollback();
    }
}
