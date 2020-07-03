extern crate crossterm;

const UPDATES_PER_SECONDS: u64 = 6;
const UPDATE_SPEED: u64 = 1000 / UPDATES_PER_SECONDS;

use crossterm::{cursor, event, terminal, QueueableCommand};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

mod game;
mod map;

fn main() {
    let stdout = stdout();
    let map = map::Map::new(70, 70);
    let game = game::Game::new(stdout, map);

    run(game);
}

fn run(mut game: game::Game) {
    // Set up terminal
    game.stdout.queue(terminal::EnterAlternateScreen).unwrap();
    game.stdout.queue(cursor::Hide).unwrap();
    game.stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
    terminal::enable_raw_mode().unwrap();
    stdout().flush().unwrap();

    let start_time = Instant::now();
    let mut next_time = start_time.elapsed().as_millis() as u64;

    game.running = true;
    while game.running {
        let current_time = start_time.elapsed().as_millis() as u64;
        if current_time >= next_time {
            next_time += UPDATE_SPEED;
            // Handle input
            while let Ok(true) = event::poll(Duration::from_millis(100)) {
                match event::read().unwrap() {
                    // Key input
                    event::Event::Key(event) => game.process_input(event.code),
                    // Terminal resize
                    event::Event::Resize(width, height) => {
                        game.resize_viewport(width as usize, height as usize)
                    }
                    _ => (),
                }
            }

            // Update
            game.update();

            // Render
            //game.stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
            game.render_map();
            game.stdout.flush().unwrap();
        } else {
            sleep(Duration::from_millis(next_time - current_time));
        }
    }
    stop();
}

fn stop() {
    // Restore terminal after game is finished
    stdout().queue(cursor::Show).unwrap();
    stdout().queue(terminal::LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();
    stdout().flush().unwrap();
    println!("Game exited successfully");
}
