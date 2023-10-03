use std::{collections::HashMap, sync::Arc};

use salvo::websocket::Message;
use snake::types::Direction;
use tokio::sync::{mpsc, RwLock};

pub mod websocket_game;

pub type User = mpsc::UnboundedSender<Result<Message, salvo::Error>>;
pub type Users = Arc<RwLock<HashMap<u16, User>>>;

pub type Directions = Arc<RwLock<HashMap<u16, Arc<RwLock<Direction>>>>>;
