pub mod system;

use bevy::prelude::*;
use system::{init, matchbox::{start_matchbox_socket, wait_for_players}};

#[derive(Debug)]
pub struct Prog;

impl Plugin for Prog {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (init, start_matchbox_socket,wait_for_players));
    }
}
