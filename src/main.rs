extern crate crossterm;

const UPDATES_PER_SECONDS: u64 = 2;
const UPDATE_SPEED: u64 = 1000 / UPDATES_PER_SECONDS;

use crossterm::{cursor, style::Print, terminal, ExecutableCommand, QueueableCommand};
use std::io::stdout;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub mod game;
mod input;

fn main() {
    let map = game::Map::new(10, 10);
    let game = game::Game::new(map);

    run(game);
}

fn run(mut game: game::Game) {
    // Set up terminal
    stdout().execute(terminal::EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();
    stdout().execute(cursor::Hide).unwrap();
    stdout()
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();

    let start_time = Instant::now();
    let mut next_time = start_time.elapsed().as_millis() as u64;

    let mut update_count = 0;
    let mut render_count = 0;

    game.running = true;
    while game.running {
        let current_time = start_time.elapsed().as_millis() as u64;
        if current_time >= next_time {
            next_time += UPDATE_SPEED;
            // Handle input
            input::handle_input(&mut game);

            // Update
            update_count += 1;

            // Render
            if current_time < next_time {
                render_count += 1;
                stdout()
                    .queue(cursor::MoveTo(0, 0))
                    .unwrap()
                    .queue(Print(game.map.get_map_string()))
                    .unwrap();

                //stdout()
                    //.queue(cursor::MoveTo(0, 0))
                    //.unwrap()
                    //.queue(Print(format!("Updates: {}", update_count)))
                    //.unwrap();
                //stdout()
                    //.queue(cursor::MoveTo(0, 1))
                    //.unwrap()
                    //.execute(Print(format!("Renders: {}", render_count)))
                    //.unwrap();
            }
        } else {
            sleep(Duration::from_millis(next_time - current_time));
        }
    }
    stop();
}

fn stop() {
    // Restore terminal after game is finished
    stdout().execute(cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
    stdout().execute(terminal::LeaveAlternateScreen).unwrap();
    println!("Game exited successfully");
}
