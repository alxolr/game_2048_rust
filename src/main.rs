mod game;
use std::io;
use std::io::Write;
use std::thread;
use std::time;

use game::{Action, Game};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{self, event::Key};

fn main() {
    let mut game = Game::new();
    println!("{}", game);

    // Set terminal to raw mode to allow reading stdin one key at a time
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    // Use asynchronous stdin
    let mut stdin = termion::async_stdin().keys();

    loop {
        // Read input (if any)
        let input = stdin.next();

        // If a key was pressed
        if let Some(Ok(key)) = input {
            match key {
                Key::Char('q') => break,

                Key::Up => game.compute(Action::Up),
                Key::Down => game.compute(Action::Down),
                Key::Left => game.compute(Action::Left),
                Key::Right => game.compute(Action::Right),

                _ => {
                    write!(
                        stdout,
                        "{}{}",
                        termion::clear::All,
                        termion::cursor::Goto(1, 1),
                    )
                    .unwrap();
                }
            }
            write!(
                stdout,
                "{}{}{}",
                termion::clear::All,
                termion::cursor::Goto(1, 1),
                game,
            )
            .unwrap();
        }
        thread::sleep(time::Duration::from_millis(50));
    }
}
