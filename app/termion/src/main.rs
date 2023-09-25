use app_termion::render::TermionRender;
use app_termion::threads::input;
use app_termion::threads::ticker;
use snake::game::*;
use snake::types::Direction;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

const CONFIG: GameConfig = GameConfig {
    size: 8,
    start: (0, 9),
    dim: (20, 10),
    direction: Direction::Right,
};

fn main() {
    let mut game_render = TermionRender::new();
    let game = Game::new(&mut game_render, CONFIG);

    let game_arc = Arc::new(RwLock::new(game));
    let mut handles = vec![];

    let game = game_arc.clone();
    handles.push(thread::spawn(move || input::read(game)));

    let game = game_arc.clone();
    handles.push(thread::spawn(move || ticker::run(game, &mut game_render)));

    for handle in handles {
        handle.join().unwrap();
    }
}
