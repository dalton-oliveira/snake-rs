use std::{sync::Arc, thread, time::Duration};

use crate::{Directions, Users};
use bincode::{config, encode_to_vec};
use futures_util::{FutureExt, StreamExt};

use salvo::websocket::{Message, WebSocket};
use snake::{
    game::Game,
    types::{Direction, GameConfig, GameState},
};
use tokio::{
    sync::{mpsc, RwLock},
    time::Instant,
};
use tokio_stream::wrappers::UnboundedReceiverStream;

const CONFIG: GameConfig = GameConfig {
    size: 5,
    start: (1, 0),
    dim: (20, 14),
    direction: Direction::Right,
};
const TICK_INTERVAL: u128 = 500;

pub struct WsGame {
    pub game: Arc<RwLock<Game>>,
    users: Users,
    directions: Directions,
}

impl WsGame {
    pub fn new() -> WsGame {
        let game = Game::new(CONFIG);
        let users = Arc::clone(&Users::default());
        let game = Arc::new(RwLock::new(game));
        WsGame {
            game,
            users,
            directions: Directions::default(),
        }
    }

    pub async fn add_user(&self, ws: WebSocket) {
        let snake_id = RwLock::write(&self.game).await.add_snake();
        let (user_ws_tx, mut user_ws_rx) = ws.split();

        let (tx, rx) = mpsc::unbounded_channel();
        let rx = UnboundedReceiverStream::new(rx);
        let fut = rx.forward(user_ws_tx).map(|result| {
            if let Err(e) = result {
                println!("websocket send error: {:?}", e);
            }
        });
        tokio::task::spawn(fut);
        let directions = Arc::clone(&self.directions);

        RwLock::write(&directions)
            .await
            .insert(snake_id, Arc::new(RwLock::new(CONFIG.direction.clone())));

        let users = Arc::clone(&self.users);
        let game = Arc::clone(&self.game);
        let fut = async move {
            users.write().await.insert(snake_id, tx);
            let fut = async move {
                WsGame::notify_snake_id(Arc::clone(&users), snake_id).await;
            };
            tokio::task::spawn(fut);

            let direction_arc = Arc::clone(RwLock::read(&directions).await.get(&snake_id).unwrap());
            let mut direction = direction_arc.read().await.clone();
            while let Some(result) = user_ws_rx.next().await {
                match result {
                    Ok(msg) => {
                        // @todo key buffer ?
                        if let Ok(msg) = msg.to_str() {
                            let next_direction = match msg {
                                "KeyI" => Some(Direction::Up),
                                "KeyJ" => Some(Direction::Left),
                                "KeyK" => Some(Direction::Down),
                                "KeyL" => Some(Direction::Right),
                                _ => None,
                            };
                            if let Some(next_direction) = next_direction {
                                if direction == next_direction {
                                    continue;
                                }
                                let mut d = direction_arc.write().await;
                                *d = next_direction;
                                direction = next_direction;
                            }
                        }
                    }
                    Err(_e) => {
                        // ignoring for now
                        break;
                    }
                };
            }
            game.write().await.remove_snake(snake_id);
        };
        tokio::task::spawn(fut);
    }

    pub fn start_game(&self) {
        let game = Arc::clone(&self.game);
        let users = Arc::clone(&self.users);
        let directions = Arc::clone(&self.directions);

        let fut = async move {
            if game.read().await.state != GameState::None {
                return;
            }
            game.write().await.state = GameState::Playing;
            let mut now = Instant::now();
            loop {
                let sleep = (TICK_INTERVAL - now.elapsed().as_millis()) as u64;
                if sleep > 0 {
                    thread::sleep(Duration::from_millis(sleep));
                }
                {
                    let mut game = game.write().await;
                    // reflect received directions on game
                    for (snake_id, direction) in directions.read().await.iter() {
                        let direction = direction.read().await.clone();
                        game.head_to(*snake_id, direction.clone());
                    }
                    game.tick();
                    game.add_missing_food();

                    if game.state == GameState::Quit {
                        break;
                    }
                }
                WsGame::send_game_data(Arc::clone(&users), Arc::clone(&game));
                now = Instant::now();
            }
        };

        tokio::task::spawn(fut);
    }

    pub async fn notify_snake_id(users: Users, snake_id: u16) {
        let mut set_snake_id: Vec<u8> = vec![2];
        set_snake_id.extend_from_slice(&encode_to_vec(snake_id, config::standard()).unwrap());
        if let Some(tx) = users.read().await.get(&snake_id) {
            tx.send(Ok(Message::binary(set_snake_id))).unwrap();
        }
    }

    pub fn send_game_data(users: Users, game: Arc<RwLock<Game>>) {
        let fut = async move {
            let mut data: Vec<u8> = vec![1];
            data.extend_from_slice(&Arc::clone(&game).read().await.encode_game_data());

            let content = Message::binary(data);
            for (_, tx) in users.read().await.iter() {
                tx.send(Ok(content.clone())).unwrap();
            }
        };
        tokio::task::spawn(fut);
    }
}
