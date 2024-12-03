pub mod system;
pub mod component;

use bevy::prelude::*;
use bevy_ggrs::prelude::*;
use system::{
    init,
    matchbox::{start_matchbox_socket, wait_for_players, Config},
};

#[derive(Debug)]
pub struct Prog;

impl Plugin for Prog {
    fn build(&self, app: &mut App) {
        app.add_plugins((GgrsPlugin::<Config>::default(),))
            .add_systems(Startup, (init, start_matchbox_socket, wait_for_players));
    }
}
