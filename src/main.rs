extern crate crossterm;

use crossterm::{cursor, event, terminal, QueueableCommand};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

mod game;
mod map;
mod util;

const TICKS_PER_SECOND: usize = 8;

fn main() {
    let stdout = stdout();
    let map = map::Map::new(200, 200);

    let game = game::Game::new(stdout, map);

    run(game);
}

fn run(mut game: game::Game) {
    // Set up terminal
    set_up_terminal(&mut game);

    let mut last_tick = Instant::now();
    let tps_nanos: f32 = (1.0 / TICKS_PER_SECOND as f32) * 1000000000.0;

    //let mut game_loop = GameLoop::new(10);
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

        // Ensure loop is running at correct speed
        // Calculates time since last tick and sleeps the thread for that amount of time
        let time = last_tick.elapsed();
        let total_nanos: f32 = time.as_secs() as f32 * 1000000000.0 + time.subsec_nanos() as f32;

        let nano_difference = tps_nanos - total_nanos;
        if nano_difference > 0.0 {
            sleep(Duration::from_nanos(nano_difference as u64));
        }

        last_tick = Instant::now();
    }

    stop(game);
}

fn set_up_terminal(game: &mut game::Game) {
    game.stdout.queue(terminal::EnterAlternateScreen).unwrap();
    game.stdout.queue(cursor::Hide).unwrap();
    game.stdout
        .queue(terminal::Clear(terminal::ClearType::All))
        .unwrap();
    game.stdout.queue(event::EnableMouseCapture).unwrap();
    terminal::enable_raw_mode().unwrap();
    stdout().flush().unwrap();
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
