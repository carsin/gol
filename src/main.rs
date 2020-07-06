extern crate crossterm;

use crossterm::{cursor, event, terminal, QueueableCommand};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

mod game;
mod map;
mod input;
mod util;

const UPDATES_PER_SECONDS: f32 = 8.0;
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

    let input_receiver = input::start_input_receiver();

    game.running = true;
    while game.running {
        let current_time = start_time.elapsed().as_nanos() as f32;
        if current_time >= next_time {
            next_time += UPDATE_SPEED;
            // Handle key input
            while let Ok(char) = input_receiver.try_recv() {
                game.process_key_input(char);
            }

            // Handle other input
            while let Ok(true) = event::poll(Duration::from_millis(10)) {
                match event::read().unwrap() {
                    // Mouse events
                    event::Event::Mouse(mouse_event) => game.process_mouse_input(mouse_event),
                    // Terminal resize
                    event::Event::Resize(width, height) => {
                        game.resize_viewport(width as usize, height as usize)
                    },
                    _ => ()
                }
            }

            // Update
            if !game.paused {
                game.update();
            }

            // Render
            //if current_time > next_time {
                game.render_status();
                game.render_map();
                game.stdout.flush().unwrap();
            //}
        } else {
            let sleep_time = (next_time - current_time) as u64;
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
