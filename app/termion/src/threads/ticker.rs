use std::{
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
};

use snake::{game::Game, render::GameRender, types::GameState};

pub fn run(game: Arc<RwLock<Game>>, game_render: &mut impl GameRender) {
    game.write().expect("errro").add_food(game_render);

    loop {
        let now = Instant::now();

        let mut game = game.write().expect("cant move");
        game.tick(game_render);

        if game.state == GameState::Quit {
            break;
        }
        let sleep = (100 - now.elapsed().as_millis()) as u64;
        if sleep > 0 {
            thread::sleep(Duration::from_millis(sleep));
        }
    }
}
