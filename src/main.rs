extern crate crossterm;

use crossterm::{cursor, event, terminal, QueueableCommand};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

mod game;
mod map;

const UPDATES_PER_SECONDS: f32 = 6.0;
const UPDATE_SPEED: f32 = 1000.0 / UPDATES_PER_SECONDS;

pub struct TimeStep {
    last_time: Instant,
    delta_time: f32,
    frame_time: f32,
    frames: usize,
}

impl TimeStep {
    pub fn new() -> TimeStep {
        TimeStep {
            last_time: Instant::now(),
            delta_time: 0.0,
            frame_time: 0.0,
            frames: 0,
        }
    }

    pub fn get_delta_time(&mut self) -> f32 {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(self.last_time).as_micros() as f32 * 0.001;
        self.last_time = current_time;
        self.delta_time = delta_time;

        delta_time
    }

    pub fn get_fps(&mut self) -> Option<usize> {
        self.frames += 1;
        self.frame_time += self.delta_time;

        if self.frame_time >= 1000.0 {
            let fps = self.frames;
            self.frames = 0;
            self.frame_time = 0.0;

            Some(fps)
        } else {
            None
        }
    }
}

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

    let mut timestep = TimeStep::new();
    let mut lag = 0.0;

    game.running = true;
    loop {
        if !game.running { break };

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


        lag += timestep.get_delta_time();

        while lag >= UPDATE_SPEED {
            // Update
            if !game.paused {
                game.update();
            }

            lag -= UPDATE_SPEED;
        }

        // Render
        game.render_status();
        game.render_map();
        game.stdout.flush().unwrap();
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
