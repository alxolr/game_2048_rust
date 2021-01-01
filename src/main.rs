extern crate termion;
mod game;

use game::{Action, Game};
use std::io::{stdin, stdout};
use termion::event::{Event, Key};
use termion::input::TermRead;

fn main() {
    let mut game = Game::new();
    let stdin = stdin();

    println!("{}", game);

    for c in stdin.events() {
        let event = c.unwrap();
        match event {
            Event::Key(Key::Up) => {
                game.compute(Action::Up);
                println!("{}", game);
            }
            Event::Key(Key::Char('q')) => {
                println!("exit game");
                break;
            }
            _ => {}
        }
    }

}
