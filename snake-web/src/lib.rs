use std::{collections::HashMap, sync::Arc};

use snake::types::Direction;
use tokio::sync::RwLock;
pub mod input_thread;
pub mod websocket_game;

pub type DirectionArc = Arc<RwLock<Direction>>;
pub type Directions = Arc<RwLock<HashMap<u16, DirectionArc>>>;
