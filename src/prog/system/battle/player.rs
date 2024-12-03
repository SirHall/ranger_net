use bevy::prelude::*;
use bevy_ggrs::PlayerInputs;

use crate::prog::{component::player::Player, system::{
    inputs::{INPUT_DOWN, INPUT_LEFT, INPUT_RIGHT, INPUT_UP},
    matchbox::Config,
}};

pub fn move_players(inputs: Res<PlayerInputs<Config>>, mut players: Query<&mut Transform, With<Player>>) {
    let mut direction = Vec2::ZERO;

    let (input, _) = inputs[0];

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
        return;
    }

    let move_speed = 0.13;
    let move_delta = (direction * move_speed).extend(0.);

    for mut transform in &mut players {
        transform.translation += move_delta;
    }
}
