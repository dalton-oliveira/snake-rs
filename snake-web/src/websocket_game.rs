use crate::input_thread::rx_commands;
use futures_util::SinkExt;
use futures_util::StreamExt;
use salvo::websocket::{Message, WebSocket};
use snake::{
    game::Game,
    types::{Direction, GameConfig, GameState},
    utils::encode,
};
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::watch;
use tokio::{
    sync::{
        watch::{Receiver, Sender},
        RwLock,
    },
    time::{sleep, Instant},
};
use tracing::error;
use tracing::error_span;
use tracing::info_span;
use tracing::span;
use tracing::Level;

const CONFIG: GameConfig = GameConfig {
    size: 5,
    start: (1, 0),
    dim: (30, 20),
    direction: Direction::Right,
};
const TICK_INTERVAL: u128 = 251 * 1000;
#[derive(Debug)]
pub struct WsGame {
    pub game: Arc<RwLock<Game>>,
    game_data_sender: Arc<RwLock<Sender<Message>>>,
    game_data_receiver: Receiver<Message>,
}

pub const GAME_DATA: u8 = 1;
pub const NOTIFY: u8 = 2;
pub const PING: u8 = 3;
pub const DIRECTION: u8 = 4;

impl Default for WsGame {
    fn default() -> Self {
        let game = Game::new(CONFIG);
        let game = Arc::new(RwLock::new(game));
        let (game_data_sender, game_data_receiver) = watch::channel(Message::binary(vec![]));

        WsGame {
            game,
            game_data_sender: Arc::new(RwLock::new(game_data_sender)),
            game_data_receiver,
        }
    }
}

impl WsGame {
    pub async fn ingress_user(&self, ws: WebSocket) {
        let (mut ws_tx, ws_rx) = ws.split();

        let snake_id = RwLock::write(&self.game).await.add_snake();
        let game = Arc::clone(&self.game);
        let mut game_data_receiver = self.game_data_receiver.clone();
        tokio::task::spawn(async move {
            let notify = Message::binary(to_command(NOTIFY, encode(snake_id).unwrap()));
            if let Err(_msg) = ws_tx.send(notify).await {
                RwLock::write(&game).await.remove_snake(snake_id);
            }
            while let Ok(()) = game_data_receiver.changed().await {
                let loop_span = span!(Level::INFO, "game_data", snake_id);
                let _enter = loop_span.enter();

                let game_span = info_span!("game_data");
                let game_data = game_data_receiver.borrow_and_update().to_owned();

                if let Err(_msg) = ws_tx.send(game_data).await {
                    error_span!("game_data");
                    break;
                }
                drop(game_span);

                let ping_span = info_span!("ping");
                let ping = Message::binary(to_command(PING, encode(SystemTime::now()).unwrap()));
                if let Err(_msg) = ws_tx.send(ping).await {
                    // @todo how to send error spans
                    span!(Level::ERROR, "ping_error");
                    break;
                }
                drop(ping_span);
            }

            let span = span!(Level::INFO, "remove", snake_id);

            RwLock::write(&game).await.remove_snake(snake_id);
            drop(span);
        });

        rx_commands(snake_id, ws_rx, Arc::clone(&self.game))
    }

    pub fn start_game(&self) {
        let game_arc = Arc::clone(&self.game);
        let game_data_sender = Arc::clone(&self.game_data_sender);
        let fut = async move {
            {
                let mut game = RwLock::write(&game_arc).await;
                if game.state != GameState::None {
                    // panic!("can't start game again")
                    return;
                }
                game.state = GameState::Playing;
            }
            loop {
                let now = Instant::now();
                {
                    let root = info_span!("game_loop");
                    let _enter = root.enter();

                    let mut game = RwLock::write(&game_arc).await;
                    if game.state == GameState::Over {
                        break;
                    }

                    let span = info_span!("game_tick");
                    game.tick();
                    game.add_missing_food();
                    drop(span);

                    let span = info_span!("encode_game_data");
                    let game_data = game.encode_game_data();
                    let game_data = Message::binary(to_command(GAME_DATA, game_data));
                    drop(span);

                    let span = info_span!("send_game_data");
                    let game_data_sender = RwLock::write(&game_data_sender).await;
                    if let Err(msg) = game_data_sender.send(game_data) {
                        error!("error sending game_data {msg:?}");
                    }
                    drop(span);
                }

                let elapsed_micro = now.elapsed().as_micros();
                if let Some(sleep_micros) = TICK_INTERVAL.checked_sub(elapsed_micro) {
                    sleep(Duration::from_micros(sleep_micros as u64)).await;
                }
            }
        };

        tokio::spawn(fut);
    }
}

fn to_command(id: u8, bytes: Vec<u8>) -> Vec<u8> {
    let mut data: Vec<u8> = vec![id];
    data.extend_from_slice(&bytes);
    data
}
