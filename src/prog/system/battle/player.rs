use crate::prog::{component::player::Player, system::matchbox::Config};
use avian2d::prelude::*;
use bevy::{color::palettes::css, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_ggrs::{AddRollbackCommandExtension, Session};
use bevy_mod_raycast::prelude::*;

pub fn spawn_players(
    mut commands: Commands,
    session: Res<Session<Config>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let num_players = match &*session {
        Session::SyncTest(s) => s.num_players(),
        Session::P2P(s) => s.num_players(),
        Session::Spectator(s) => s.num_players(),
    };

    let mesh = meshes.add(Circle::default());
    let material = materials.add(ColorMaterial::from(Color::from(css::BLUE)));

    for i in 0..num_players {
        commands
            .spawn((
                Player { handle: i },
                MaterialMesh2dBundle {
                    mesh: mesh.clone().into(),
                    material: material.clone(),
                    transform: Transform::from_translation(Vec3::new(0. + (i as f32), 0., 1.)),
                    ..Default::default()
                },
                RaycastMesh::<()>::default(),
                // NOTE: Not used right now
                RigidBody::Dynamic,
                Collider::circle(1.0),
            ))
            .add_rollback();
    }
}
