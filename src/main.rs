extern crate crossterm;

use crossterm::{cursor, event, terminal, QueueableCommand};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

mod game;
mod map;
mod util;

fn main() {
    let stdout = stdout();
    let map = map::Map::new(200, 200);

    let game = game::Game::new(stdout, map);

    run(game);
}

struct GameLoop {
    last_tick: Instant,
    tps: usize,
    tps_nanos: f32,
}

impl GameLoop {
    pub fn new(tps: usize) -> GameLoop {
        GameLoop {
            last_tick: Instant::now(),
            tps,
            tps_nanos: (1.0 / tps as f32) * 1000000000.0,
        }
    }

    // Slows down the game loop
    pub fn tick(&mut self) -> f32 {
        let time = self.last_tick.elapsed();
        let total_nanos: f32 = time.as_secs() as f32 * 1000000000.0 + time.subsec_nanos() as f32;

        let nano_difference = self.tps_nanos - total_nanos;
        if nano_difference > 0.0 {
            sleep(Duration::from_nanos(nano_difference as u64));
        }

        self.last_tick = Instant::now();
        nano_difference
    }
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

    let mut game_loop = GameLoop::new(6);

    game.running = true;
    while game.running {
        // Handle input
        while let Ok(true) = event::poll(Duration::from_millis(10)) {
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
        game.stdout.queue(cursor::Hide).unwrap();
        game.render_status();
        game.render_map();
        game.stdout.flush().unwrap();

        game_loop.tick();
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
