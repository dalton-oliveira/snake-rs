extern crate termion;

use std::io::stdin;
use std::sync::{Arc, RwLock};

use snake::game::Game;
use snake::types::{Direction, GameState};
use termion::event::Key;
use termion::input::TermRead;

pub fn read(game_arc: Arc<RwLock<Game>>) {
    let snake_id = RwLock::write(&game_arc)
        .expect("can't add snake")
        .add_snake();
    let mut stdin = stdin().lock().keys();

    loop {
        let game = Arc::clone(&game_arc);
        let key = stdin.next();
        match key.unwrap().unwrap() {
            Key::Char('q') | Key::Esc => {
                quit(game);
                break;
            }
            Key::Left => head_move(game, snake_id, Direction::Left),
            Key::Up => head_move(game, snake_id, Direction::Up),
            Key::Right => head_move(game, snake_id, Direction::Right),
            Key::Down => head_move(game, snake_id, Direction::Down),
            _ => (),
        }
    }
}

fn head_move(game: Arc<RwLock<Game>>, snake_id: u16, to: Direction) {
    RwLock::write(&game)
        .expect("can't change head")
        .head_to(snake_id, to);
}

fn quit(game: Arc<RwLock<Game>>) {
    let mut game = RwLock::write(&game).expect("can't quit");
    game.state = GameState::Quit;
}
