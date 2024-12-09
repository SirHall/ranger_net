use bevy::prelude::*;
use bevy_ggrs::*;
use bevy_matchbox::prelude::*;

use crate::prog::resource::game_state::GameState;

// The first generic parameter, u8, is the input type: 4-directions + fire fits
// easily in a single byte
// The second parameter is the address type of peers: Matchbox' WebRtcSocket
// addresses are called `PeerId`s
pub type InputWord = u32;
pub type Config = bevy_ggrs::GgrsConfig<InputWord, PeerId>;

pub fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://127.0.0.1:3536/ranger_net?next=2";
    info!("connecting to matchbox server: {room_url}");
    commands.insert_resource(MatchboxSocket::new_ggrs(room_url));
}

pub fn wait_for_players(
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>, // <-- NEW
) {
    if socket.get_channel(0).is_err() {
        return; // we've already started
    }

    // Check for new connections
    socket.update_peers();
    let players = socket.players();

    let num_players = 2;
    if players.len() < num_players {
        return; // wait for more players
    }

    info!("All peers have joined, going in-game");

    // create a GGRS P2P session
    let mut session_builder = ggrs::SessionBuilder::<Config>::new()
        .with_num_players(num_players)
        .with_input_delay(2);

    for (i, player) in players.into_iter().enumerate() {
        session_builder = session_builder.add_player(player, i).expect("failed to add player");
    }

    // move the channel out of the socket (required because GGRS takes ownership of it)
    let channel = socket.take_channel(0).unwrap();

    // start the GGRS session
    let ggrs_session = session_builder
        .start_p2p_session(channel)
        .expect("failed to start session");

    commands.insert_resource(bevy_ggrs::Session::P2P(ggrs_session));

    next_state.set(GameState::Game);
    println!("Setting up game");
}
