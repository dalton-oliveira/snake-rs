use futures_util::stream::SplitStream;
use futures_util::StreamExt;
use salvo::websocket::WebSocket;
use snake::{
    game::Game,
    types::{Direction, GameState},
    utils::decode,
};
use std::{sync::Arc, time::SystemTime};
use tokio::sync::RwLock;
use tracing::debug;

use crate::{
    websocket_game::{PONG, QUIT},
    DirectionArc,
};

pub fn rx_commands(
    direction_arc: DirectionArc,
    snake_id: u16,
    mut rx: SplitStream<WebSocket>,
    game: Arc<RwLock<Game>>,
) {
    let fut = async move {
        let mut direction = direction_arc.read().await.clone();
        while let Some(result) = rx.next().await {
            match result {
                Err(_msg) => break,
                Ok(msg) => {
                    let msg = msg.as_bytes();
                    if msg.is_empty() {
                        continue;
                    }
                    // @todo make first u8 always the type
                    if msg[0] == PONG {
                        log_ping(msg, snake_id);
                        continue;
                    }

                    if msg[0] == QUIT {
                        RwLock::write(&game).await.state = GameState::Over;
                        break;
                    }

                    if let Some(next_direction) = to_direction(msg[0]) {
                        if direction == next_direction {
                            continue;
                        }
                        let mut d = RwLock::write(&direction_arc).await;
                        *d = next_direction;
                        direction = next_direction;
                    }
                }
            }
        }
        RwLock::write(&game).await.remove_snake(snake_id);
    };
    tokio::task::spawn(fut);
}

fn log_ping(msg: &[u8], snake_id: u16) {
    let (past, _size): (SystemTime, usize) = decode(&msg[1..msg.len()]).unwrap();
    let μs = SystemTime::now()
        .duration_since(past)
        .expect("μs opa!")
        .as_micros();
    debug!("snake: {snake_id} ping: {:?} μs", μs);
}

fn to_direction(msg: u8) -> Option<Direction> {
    match msg {
        0 => Some(Direction::Left),
        1 => Some(Direction::Up),
        2 => Some(Direction::Right),
        3 => Some(Direction::Down),
        _ => None,
    }
}
