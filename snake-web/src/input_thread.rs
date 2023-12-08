use futures_util::stream::SplitStream;
use futures_util::StreamExt;
use salvo::websocket::WebSocket;
use snake::{game::Game, types::Direction, utils::decode};
use std::{sync::Arc, time::SystemTime};
use tokio::sync::RwLock;
use tracing::{instrument, Span};

use crate::{websocket_game::DIRECTION, websocket_game::PING};

#[instrument(skip_all)]
pub fn rx_commands(snake_id: u16, mut rx: SplitStream<WebSocket>, game: Arc<RwLock<Game>>) {
    let fut = async move {
        while let Some(result) = rx.next().await {
            let now = SystemTime::now();
            match result {
                Err(_msg) => break,
                Ok(msg) => {
                    let msg = msg.as_bytes();
                    if msg.is_empty() {
                        continue;
                    }
                    match msg {
                        [PING, ..] => log_ping(msg, now, snake_id),
                        [DIRECTION, code] => {
                            if let Some(next_direction) = to_direction(*code) {
                                RwLock::write(&game).await.head_to(snake_id, next_direction);
                            }
                        }
                        _ => continue,
                    }
                }
            }
        }
        RwLock::write(&game).await.remove_snake(snake_id);
    };
    tokio::task::spawn(fut);
}

#[instrument(fields(ping_ms, ping_μs) skip(msg, now))]
fn log_ping(msg: &[u8], now: SystemTime, snake_id: u16) {
    let (past, _size): (SystemTime, usize) = decode(&msg[1..msg.len()]).unwrap();
    let duration = now.duration_since(past).expect("ping measure error");
    Span::current().record("ping_ms", duration.as_millis());
    Span::current().record("ping_μs", duration.as_micros());
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
