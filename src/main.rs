extern crate crossterm;

use crossterm::{cursor, event, terminal, QueueableCommand};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

mod game;
mod map;

const UPDATES_PER_SECONDS: f32 = 10.0;
const UPDATE_SPEED: f32 = 1000.0 / UPDATES_PER_SECONDS;

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
    let mut next_time = start_time.elapsed().as_nanos() as f32;

    game.running = true;
    while game.running {
        let current_time = start_time.elapsed().as_nanos() as f32;
        if current_time >= next_time {
            next_time += UPDATE_SPEED;
            // Handle input
            while let Ok(true) = event::poll(Duration::from_millis(UPDATE_SPEED as u64)) {
                match event::read().unwrap() {
                    // Key input
                    event::Event::Key(key_event) => game.process_key_input(key_event.code),
                    event::Event::Mouse(mouse_event) => game.process_mouse_input(mouse_event),
                    // Terminal resize
                    event::Event::Resize(width, height) => {
                        game.resize_viewport(width as usize, height as usize)
                    },
                }
            }

            // Update
            if !game.paused {
                game.update();
            }

            // Render
            game.render_status();
            game.render_map();
            game.stdout.flush().unwrap();
        } else {
            let sleep_time = (next_time - current_time) as u64;
            println!("{}", sleep_time);
            sleep(Duration::from_nanos(sleep_time));
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
