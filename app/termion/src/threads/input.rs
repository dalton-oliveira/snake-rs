extern crate termion;

use std::io::stdin;
use std::sync::{Arc, RwLock};

use snake::game::Game;
use snake::types::{Direction, GameState};
use termion::event::Key;
use termion::input::TermRead;

pub fn read(game_arc: Arc<RwLock<Game>>, snake_id: usize) {
    let mut stdin = stdin().lock().keys();
    loop {
        let key = stdin.next();
        match key.unwrap().unwrap() {
            Key::Char('q') | Key::Esc => {
                quit(game_arc.clone());
                break;
            }
            Key::Left => head_move(game_arc.clone(), snake_id, Direction::Left),
            Key::Up => head_move(game_arc.clone(), snake_id, Direction::Up),
            Key::Right => head_move(game_arc.clone(), snake_id, Direction::Right),
            Key::Down => head_move(game_arc.clone(), snake_id, Direction::Down),
            _ => (),
        }
    }
}

fn head_move(game_arc: Arc<RwLock<Game>>, snake_id: usize, to: Direction) {
    game_arc
        .write()
        .expect("can't change head")
        .head_to(snake_id, to);
}

fn quit(game_arc: Arc<RwLock<Game>>) {
    let mut game = game_arc.write().expect("cant write");
    game.state = GameState::Quit;
}
