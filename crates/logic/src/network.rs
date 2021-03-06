use crate::input::INPUT_SIZE;
use crate::states::GameState;
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use bevy_ggrs::CommandsExt;
use ggrs::P2PSession;
use ggrs::PlayerType;
use matchbox_socket::WebRtcNonBlockingSocket;

pub(crate) fn start_matchbox_socket(mut commands: Commands, task_pool: Res<IoTaskPool>) {
    let room_url = "ws://matchbox-vrixyz.herokuapp.com/next_2";
    info!("connecting to matchbox server: {:?}", room_url);
    let (socket, message_loop) = WebRtcNonBlockingSocket::new(room_url);

    // The message loop needs to be awaited, or nothing will happen.
    // We do this here using bevy's task system.
    task_pool.spawn(message_loop).detach();

    commands.insert_resource(Some(socket));
}

pub(crate) fn wait_for_players(
    mut commands: Commands,
    mut socket: ResMut<Option<WebRtcNonBlockingSocket>>,
    mut state: ResMut<State<GameState>>,
) {
    let socket = socket.as_mut();

    // If there is no socket we've already started the game
    if socket.is_none() {
        return;
    }

    // Check for new connections
    socket.as_mut().unwrap().accept_new_connections();
    let players = socket.as_ref().unwrap().players();

    let num_players = 2;
    if players.len() < num_players {
        return; // wait for more players
    }

    info!("All peers have joined, going in-game");
    info!("All peers have joined, going in-game");

    // consume the socket (currently required because GGRS takes ownership of its socket)
    let socket = socket.take().unwrap();

    let max_prediction = 12;

    // create a GGRS P2P session
    let mut p2p_session =
        ggrs::P2PSession::new_with_socket(num_players as u32, INPUT_SIZE, max_prediction, socket);

    for (i, player) in players.into_iter().enumerate() {
        p2p_session
            .add_player(player, i)
            .expect("failed to add player");

        if player == PlayerType::Local {
            // set input delay for the local player
            p2p_session.set_frame_delay(2, i).unwrap();
        }
    }

    // start the GGRS session
    commands.start_p2p_session(p2p_session);

    state.set(GameState::InGame).unwrap();
}
