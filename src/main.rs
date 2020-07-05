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
    let map = map::Map::new(500, 500);
    let game = game::Game::new(stdout, map);

    run(game);
}

fn run(mut game: game::Game) {
    // Set up terminal
    game.stdout.queue(terminal::EnterAlternateScreen).unwrap();
    game.stdout.queue(cursor::Hide).unwrap();
    game.stdout
        .queue(terminal::Clear(terminal::ClearType::All))
        .unwrap();
    game.stdout.queue(event::EnableMouseCapture).unwrap();
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
            while let Ok(true) = event::poll(Duration::from_millis(50)) {
                match event::read().unwrap() {
                    // Key input
                    event::Event::Key(key_event) => game.process_key_input(key_event.code),
                    event::Event::Mouse(mouse_event) => game.process_mouse_input(mouse_event),
                    // TODO: Fix
                    // Terminal resize
                    //event::Event::Resize(width, height) => {
                        //game.resize_viewport(width as usize, height as usize)
                    //},
                    _ => (),
                }
            }

            // Update
            if !game.paused {
                game.update();
            }

            // Render
            game.render_map();
            game.stdout.flush().unwrap();
        } else {
            sleep(Duration::from_millis(next_time - current_time));
        }
    }

    stop(game);
}

fn stop(mut game: game::Game) {
    // Restore terminal after game is finished
    game.stdout.queue(cursor::Show).unwrap();
    game.stdout.queue(terminal::LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();
    game.stdout.queue(event::DisableMouseCapture).unwrap();
    game.stdout.flush().unwrap();
    println!("Game exited successfully");
}
