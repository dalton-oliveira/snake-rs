use snake::{game::Game, types::GameState};
use std::{
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
};

use crate::render::TermionRender;

pub fn run(game: Arc<RwLock<Game>>) {
    RwLock::write(&game).expect("can't add food").add_food();
    let mut render = TermionRender::new();
    loop {
        let now = Instant::now();

        let mut game = RwLock::write(&game).expect("cant move");
        game.tick();
        game.add_missing_food();

        render.clear();
        game.draw(&mut render);

        if game.state == GameState::Quit {
            break;
        }
        let sleep = (500 - now.elapsed().as_millis()) as u64;
        if sleep > 0 {
            thread::sleep(Duration::from_millis(sleep));
        }
    }
}
