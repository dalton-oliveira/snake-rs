use snake::game::*;
use snake::types::GameConfig;
use snake_termion::input;
use snake_termion::render::TermionRender;
use snake_termion::ticker;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

fn main() {
    let game = Game::new(GameConfig::default());

    let game_arc = Arc::new(RwLock::new(game));
    let mut handles = vec![];

    let game = Arc::clone(&game_arc);
    handles.push(thread::spawn(move || input::read(game)));

    let game = Arc::clone(&game_arc);
    handles.push(thread::spawn(move || ticker::run(game)));

    for handle in handles {
        handle.join().unwrap();
    }

    let mut render = TermionRender::default();
    render.show_cursor();
}
