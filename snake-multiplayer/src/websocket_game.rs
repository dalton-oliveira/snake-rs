use crate::{input_thread::rx_commands, DirectionArc, Directions, User, Users};
use futures_util::{stream::SplitSink, FutureExt, StreamExt};
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tracing::{info, instrument};

use salvo::{
    websocket::{Message, WebSocket},
    Error,
};
use snake::{
    game::Game,
    types::{Direction, GameConfig, GameState},
    utils::encode,
};
use tokio::{
    sync::{
        mpsc::{self, UnboundedSender},
        RwLock,
    },
    time::{sleep, Instant},
};
use tokio_stream::wrappers::UnboundedReceiverStream;

const CONFIG: GameConfig = GameConfig {
    size: 5,
    start: (1, 0),
    dim: (20, 14),
    direction: Direction::Right,
};
const TICK_INTERVAL: u128 = 300 * 1000;
#[derive(Debug)]
pub struct WsGame {
    pub game: Arc<RwLock<Game>>,
    users: Users,
    directions: Directions,
}

const GAME_DATA: u8 = 1;
const NOTIFY: u8 = 2;
const PING: u8 = 3;

impl WsGame {
    pub fn new() -> WsGame {
        let game = Game::new(CONFIG);
        let users = Users::default();
        let game = Arc::new(RwLock::new(game));
        WsGame {
            game,
            users,
            directions: Directions::default(),
        }
    }

    #[instrument]
    async fn add_user(&self, tx: UnboundedSender<Result<Message, Error>>) -> (u16, DirectionArc) {
        let direction = Arc::new(RwLock::new(CONFIG.direction.clone()));
        let mut directions = RwLock::write(&self.directions).await;

        let snake_id = RwLock::write(&self.game).await.add_snake();
        directions.insert(snake_id, Arc::clone(&direction));

        RwLock::write(&self.users).await.insert(snake_id, tx);
        notify_snake_id(Arc::clone(&self.users), snake_id).await;

        return (snake_id, direction);
    }

    #[instrument]
    pub async fn ingress_user(&self, ws: WebSocket) {
        let (ws_tx, rx) = ws.split();
        let tx = as_unbounded(ws_tx);

        let (snake_id, direction) = self.add_user(tx).await;
        rx_commands(direction, snake_id, rx, Arc::clone(&self.game));
    }

    pub fn start_game(&self) {
        let game = Arc::clone(&self.game);
        let users = Arc::clone(&self.users);
        let directions = Arc::clone(&self.directions);

        let fut = async move {
            if RwLock::read(&game).await.state != GameState::None {
                panic!("can't start game again")
            }
            RwLock::write(&game).await.state = GameState::Playing;
            let mut now = Instant::now();
            loop {
                {
                    let mut game = RwLock::write(&game).await;
                    // copy received directions on game
                    for (snake_id, direction) in RwLock::read(&directions).await.iter() {
                        let direction = RwLock::read(direction).await.clone();
                        game.head_to(*snake_id, direction);
                    }
                    game.tick();
                    game.add_missing_food();
                }

                send_game_data(Arc::clone(&users), Arc::clone(&game)).await;
                let elapsed_micro = now.elapsed().as_micros();
                info!(elapsed_micro, "elapsed_micro:");

                sleep(Duration::from_micros(
                    TICK_INTERVAL.max(elapsed_micro) as u64
                ))
                .await;
                // if elapsed_micro > TICK_INTERVAL {
                //     let sleep_time = (elapsed_micro - TICK_INTERVAL) as u64;
                // }
                now = Instant::now();
            }
        };

        tokio::task::spawn(fut);
    }
}

fn as_unbounded(ws_tx: SplitSink<WebSocket, Message>) -> UnboundedSender<Result<Message, Error>> {
    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);
    let fut = rx.forward(ws_tx).map(|result| {
        if let Err(_e) = result {
            // ignoring all errors
        }
    });
    tokio::task::spawn(fut);
    return tx;
}

#[instrument]
async fn notify_snake_id(users: Users, snake_id: u16) {
    let notify = to_command(NOTIFY, encode(snake_id).unwrap());
    if let Some(tx) = RwLock::read(&users).await.get(&snake_id) {
        if let Err(_msg) = tx.send(notify) {
            // ignoring all errors
        }
    }
}

#[instrument]
async fn send_game_data(users: Users, game: Arc<RwLock<Game>>) {
    let game_data = RwLock::read(&game).await.encode_game_data();
    let mut threads = Vec::new();

    for (&snake_id, tx) in RwLock::read(&users).await.iter() {
        let game = Arc::clone(&game);
        let tx = tx.clone();
        let command = to_command(GAME_DATA, game_data.clone());
        threads.push(tokio::task::spawn(async move {
            send_ping(tx.clone());
            if let Err(_msg) = tx.send(command) {
                RwLock::write(&game).await.remove_snake(snake_id);
            }
        }));
    }
    futures_util::future::join_all(threads).await;
}

#[instrument]
fn send_ping(user: User) {
    let ping = to_command(PING, encode(SystemTime::now()).unwrap());
    if let Err(_msg) = user.send(ping) {
        // ignore for now
    }
}

#[instrument]
fn to_command(id: u8, bytes: Vec<u8>) -> Result<Message, Error> {
    let mut data: Vec<u8> = vec![id];
    data.extend_from_slice(&bytes);
    Ok(Message::binary(data))
}
