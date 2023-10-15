use crate::{input_thread::rx_commands, DirectionArc, Directions, User, Users};
use futures_util::{stream::SplitSink, FutureExt, StreamExt};
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tracing::{debug, instrument};

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
    dim: (20, 5),
    direction: Direction::Right,
};
const TICK_INTERVAL: u128 = 300 * 1000;
#[derive(Debug)]
pub struct WsGame {
    pub game: Arc<RwLock<Game>>,
    users: Users,
    directions: Directions,
}

pub const GAME_DATA: u8 = 1;
pub const NOTIFY: u8 = 2;
pub const PING: u8 = 3;
pub const PONG: u8 = 5;
pub const QUIT: u8 = 6;

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
        let game_arc = Arc::clone(&self.game);
        let users = Arc::clone(&self.users);
        let directions = Arc::clone(&self.directions);

        let fut = async move {
            {
                let mut game = RwLock::write(&game_arc).await;
                if game.state != GameState::None {
                    panic!("can't start game again")
                }
                game.state = GameState::Playing;
            }
            loop {
                let now = Instant::now();
                {
                    let last = Instant::now();
                    let mut game = RwLock::write(&game_arc).await;
                    if game.state == GameState::Over {
                        break;
                    }
                    // copy received directions on game
                    for (snake_id, direction) in RwLock::read(&directions).await.iter() {
                        let direction = RwLock::read(direction).await.clone();
                        game.head_to(*snake_id, direction);
                    }

                    debug!("copy_directions {:?}", last.elapsed().as_micros());
                    let last = Instant::now();
                    game.tick();
                    debug!("game_tick {:?}", last.elapsed().as_micros());
                    let last = Instant::now();
                    game.add_missing_food();
                    debug!("add_missing_food {:?}", last.elapsed().as_micros());
                    let last = Instant::now();
                    let game_data = game.encode_game_data();
                    debug!("encode_game_data {:?}", last.elapsed().as_micros());

                    drop(game);

                    let last = Instant::now();
                    send_game_data(Arc::clone(&users), Arc::clone(&game_arc), game_data).await;
                    debug!("send_game_data {:?}", last.elapsed().as_micros());
                }

                let elapsed_micro = now.elapsed().as_micros();
                debug!(elapsed_micro);
                let sleep_micros = TICK_INTERVAL.max(elapsed_micro) as u64;
                sleep(Duration::from_micros(sleep_micros)).await;
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
        if let Err(_msg) = tx.send(Ok(notify)) {
            // ignoring all errors
        }
    }
}

// #[instrument]
async fn send_game_data(users: Users, game: Arc<RwLock<Game>>, game_data: Vec<u8>) {
    let last = Instant::now();
    let ping = to_command(PING, encode(SystemTime::now()).unwrap());
    let command = to_command(GAME_DATA, game_data);
    debug!("join_all {:?}", last.elapsed().as_micros());
    for (&snake_id, tx) in RwLock::read(&users).await.iter() {
        let ping = ping.clone();
        let command = command.clone();
        if let Err(_msg) = tx.send(Ok(ping)) {
            // ignore for now
        }
        if let Err(_msg) = tx.send(Ok(command)) {
            RwLock::write(&game).await.remove_snake(snake_id);
        }
    }
}

#[instrument]
fn send_ping(user: User) {
    let ping = to_command(PING, encode(SystemTime::now()).unwrap());
    if let Err(_msg) = user.send(Ok(ping)) {
        // ignore for now
    }
}

#[instrument]
fn to_command(id: u8, bytes: Vec<u8>) -> Message {
    let mut data: Vec<u8> = vec![id];
    data.extend_from_slice(&bytes);
    Message::binary(data)
}
