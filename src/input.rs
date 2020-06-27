use crossterm::event::{poll, read, Event, KeyCode};
use std::time::Duration;

pub fn handle_input(&mut game: game::Game) {
    loop {
        if poll(Duration::from_millis(500)).unwrap() {
            match read().unwrap() {
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Char('q') => game.running = false,
                        _ => (),
                    }
                },
                //Event::Mouse(event) => println!("{:?}", event),
                //Event::Resize(width, height) => println!("New size {}x{}", width, height),
                _ => (),
            }
        }
    }
}
