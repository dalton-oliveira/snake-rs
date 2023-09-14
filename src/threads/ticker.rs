use std::{
    io::{Stdout, Write},
    thread,
    time::{Duration, Instant},
};

use termion::{raw::RawTerminal, screen::AlternateScreen};

use crate::types::{GameArc, GameState};

pub fn run(game: GameArc, screen: &mut AlternateScreen<RawTerminal<Stdout>>) {
    game.write().expect("errro").add_food(screen);

    loop {
        let now = Instant::now();

        let mut game = game.write().expect("cant move");
        game.tick(screen);
        if game.state == GameState::Quit {
            break;
        }
        let head = game.snake.nodes.front().unwrap();

        write!(
            screen,
            "{}{}{},{} size:{}",
            termion::cursor::Goto(0, 40),
            termion::clear::CurrentLine,
            head.position.x,
            head.position.y,
            game.snake.nodes.len(),
        )
        .unwrap();
        screen.flush().unwrap();

        let sleep = (200 - now.elapsed().as_millis()) as u64;
        if sleep > 0 {
            thread::sleep(Duration::from_millis(sleep));
        }
    }
}
