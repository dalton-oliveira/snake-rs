use crate::{input_thread::rx_commands, DirectionArc, Directions};
use futures_util::SinkExt;
use futures_util::{stream::SplitSink, StreamExt};
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
use tracing::{error, instrument};

const CONFIG: GameConfig = GameConfig {
    size: 5,
    start: (1, 0),
    dim: (30, 20),
    direction: Direction::Right,
};
const TICK_INTERVAL: u128 = 250 * 1000;
#[derive(Debug)]
pub struct WsGame {
    pub game: Arc<RwLock<Game>>,
    directions: Directions,
    tx: Arc<RwLock<Sender<Vec<u8>>>>,
    rx: Receiver<Vec<u8>>,
}

pub const GAME_DATA: u8 = 1;
pub const NOTIFY: u8 = 2;
pub const PING: u8 = 3;
pub const DIRECTION: u8 = 4;

impl WsGame {
    pub fn new() -> WsGame {
        let game = Game::new(CONFIG);
        let game = Arc::new(RwLock::new(game));
        let data: Vec<u8> = vec![];
        let (tx, rx) = watch::channel(data);

        WsGame {
            game,
            directions: Directions::default(),
            tx: Arc::new(RwLock::new(tx)),
            rx,
        }
    }

    #[instrument]
    async fn add_user(&self, tx: &SplitSink<WebSocket, Message>) -> (u16, DirectionArc) {
        let direction = Arc::new(RwLock::new(CONFIG.direction.clone()));
        let mut directions = RwLock::write(&self.directions).await;

        let snake_id = RwLock::write(&self.game).await.add_snake();
        directions.insert(snake_id, Arc::clone(&direction));

        return (snake_id, direction);
    }

    #[instrument]
    pub async fn ingress_user(&self, ws: WebSocket) {
        let (mut ws_tx, ws_rx) = ws.split();

        let (snake_id, direction) = self.add_user(&ws_tx).await;

        let mut rx = self.rx.clone();
        tokio::task::spawn(async move {
            let notify = Message::binary(to_command(NOTIFY, encode(snake_id).unwrap()));
            if let Err(_msg) = ws_tx.send(notify).await {
                // ignoring all errors
            }
            loop {
                if let Ok(()) = rx.changed().await {
                    let ping =
                        Message::binary(to_command(PING, encode(SystemTime::now()).unwrap()));
                    if let Err(_msg) = ws_tx.send(ping).await {
                        break;
                        // ignoring all errors
                    }
                    // let last = Instant::now();
                    let val = rx.borrow_and_update().to_owned();

                    let game_data = Message::binary(to_command(GAME_DATA, val));
                    if let Err(_msg) = ws_tx.send(game_data).await {
                        break;
                        // ignoring all errors
                    }
                }
            }
        });

        rx_commands(direction, snake_id, ws_rx, Arc::clone(&self.game));
    }

    pub fn start_game(&self) {
        let game_arc = Arc::clone(&self.game);
        let directions = Arc::clone(&self.directions);
        let tx = Arc::clone(&self.tx);
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
                    let mut game = RwLock::write(&game_arc).await;
                    let tx = RwLock::write(&tx).await;
                    if game.state == GameState::Over {
                        break;
                    }

                    // copy received directions on game
                    for (snake_id, direction) in RwLock::read(&directions).await.iter() {
                        let direction = RwLock::read(direction).await.clone();
                        game.head_to(*snake_id, direction);
                    }

                    game.tick();
                    game.add_missing_food();

                    let game_data = game.encode_game_data();

                    if let Err(msg) = tx.send(game_data) {
                        error!("error sending game_data {msg:?}");
                    }
                }

                let elapsed_micro = now.elapsed().as_micros();
                if let Some(sleep_micros) = TICK_INTERVAL.checked_sub(elapsed_micro) {
                    sleep(Duration::from_micros(sleep_micros as u64)).await;
                }
            }
        };

        tokio::task::spawn(fut);
    }
}

fn to_command(id: u8, bytes: Vec<u8>) -> Vec<u8> {
    let mut data: Vec<u8> = vec![id];
    data.extend_from_slice(&bytes);
    data
}
