pub mod component;
pub mod system;

use bevy::prelude::*;
use bevy_ggrs::prelude::*;
use system::{
    battle::{
        local::{camera_follow, init_grid_map},
        player::{move_players, spawn_players},
    },
    init,
    inputs::read_local_inputs,
    matchbox::{start_matchbox_socket, wait_for_players, Config},
};

#[derive(Debug)]
pub struct Prog;

impl Plugin for Prog {
    fn build(&self, app: &mut App) {
        app.add_plugins((GgrsPlugin::<Config>::default(),))
            .add_systems(Startup, (init, start_matchbox_socket, spawn_players, init_grid_map))
            .add_systems(Update, (wait_for_players, camera_follow))
            .add_systems(ReadInputs, (read_local_inputs,))
            .add_systems(GgrsSchedule, (move_players,))
            .rollback_component_with_clone::<Transform>();
    }
}
