extern crate crossterm;

const UPDATES_PER_SECONDS: u64 = 5;
const UPDATE_SPEED: u64 = 1000 / UPDATES_PER_SECONDS;

use crossterm::{cursor, event, terminal, ExecutableCommand};
use std::io::stdout;
use std::thread::sleep;
use std::time::{Duration, Instant};

mod game;

fn main() {
    let stdout = stdout();
    let terminal_size = terminal::size();
    println!("{:?}", terminal_size);
    let map = game::Map::new(30, 30);
    let game = game::Game::new(stdout, map);

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

    game.running = true;
    while game.running {
        let current_time = start_time.elapsed().as_millis() as u64;
        if current_time >= next_time {
            next_time += UPDATE_SPEED;
            // Handle input
            while let Ok(true) = event::poll(Duration::from_millis(UPDATE_SPEED)) {
                match event::read().unwrap() {
                    // Key Input
                    event::Event::Key(event) => match event.code {
                        event::KeyCode::Char('q') => game.running = false,
                        _ => (),
                    },
                    _ => (),
                }
            }

            // Update

            // Render
            if current_time < next_time {
                game.render_map();
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
