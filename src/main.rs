extern crate termion;

use snake::game::*;
use snake::threads::input;
use snake::threads::ticker;
use std::io::stdout;
use std::io::Write;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::screen::{IntoAlternateScreen, ToAlternateScreen};

fn main() {
    let stdout = stdout()
        .into_raw_mode()
        .unwrap()
        .into_alternate_screen()
        .unwrap();

    let mut screen = AlternateScreen::from(stdout);

    write!(screen, "{}{}", termion::cursor::Hide, ToAlternateScreen).unwrap();
    screen.flush().unwrap();

    let game = Game::new(&mut screen);

    let game_arc = Arc::new(RwLock::new(game));
    let mut handles = vec![];

    let game = game_arc.clone();
    handles.push(thread::spawn(move || input::read(game)));

    let game = game_arc.clone();
    handles.push(thread::spawn(move || ticker::run(game, &mut screen)));

    for handle in handles {
        handle.join().unwrap();
    }
}
