use crossterm::{cursor, event, style::Print, terminal, QueueableCommand, ExecutableCommand};
use std::cmp::min;

use super::map;
use super::util;
use super::camera;

pub enum Direction {
    North, South, East, West,
}

pub struct Game {
    pub stdout: std::io::Stdout,
    pub map: map::Map,
    pub running: bool,
    pub paused: bool,
    pub camera: camera::Camera,
    pub update_count: usize,
    pub updates_per_second: f32,
}

impl Game {
    pub fn new(stdout: std::io::Stdout, map: map::Map) -> Self {
        let temp = [map.width, map.height];
        Self {
            stdout,
            map,
            running: false,
            paused: false,
            camera: camera::Camera::new(temp[0] / 2, temp[1] / 2),
            update_count: 0,
            updates_per_second: 5.0,
        }
    }

    pub fn update(&mut self) {
        self.map.update();
        self.update_count += 1;
    }

    pub fn render_map(&mut self) {
        for x in 0..self.camera.viewport_width {
            for y in 1..self.camera.viewport_height {
                // Calculate the position to render, None if negative (can't access negative array indicies)
                let x_pos = (x + self.camera.x).checked_sub((self.camera.viewport_width / 2) as usize);
                let y_pos = (y + self.camera.y).checked_sub((self.camera.viewport_height / 2) as usize);
                let positions = [x_pos, y_pos];

                let chars_to_print = match positions {
                    // Good input
                    [Some(x), Some(y)] => {
                        // Check if position is valid (within array)
                        if let Some(pos) = self.map.pos(x, y) {
                            match self.map.cells[pos] {
                                false => "  ",
                                true => "██",
                            }
                        } else {
                            "  "
                        }
                    }
                    // Render blank if checked_sub failed
                    _ => "  ",
                };

                self.stdout
                    .queue(cursor::MoveTo((x * 2) as u16, y as u16))
                    .unwrap()
                    .queue(Print(chars_to_print))
                    .unwrap();
            }
        }

    }

    pub fn render_status(&mut self) {
        self.stdout
            .queue(cursor::MoveTo(0, 0))
            .unwrap()
            .queue(terminal::Clear(terminal::ClearType::CurrentLine))
            .unwrap()
            .queue(Print(format!("X: {}, Y: {} ─── Camera Speed: {}", self.camera.x, self.camera.y, self.camera.speed)))
            .unwrap();
        if self.paused {
            let paused_text = "─ PAUSED ─";
            self.stdout
                .queue(cursor::MoveTo((self.camera.viewport_width - paused_text.len()) as u16, 0))
                .unwrap()
                .queue(Print(format!("{}", paused_text)))
                .unwrap();
        }

        let right_status_text = format!("Alive Cells: {} ─── Gen: {}", self.map.live_cell_count, self.update_count);
        self.stdout
            .queue(cursor::MoveTo(((self.camera.viewport_width * 2) - right_status_text.len()) as u16, 0))
            .unwrap()
            .queue(Print(format!("{}", right_status_text)))
            .unwrap();
    }

    pub fn process_mouse_input(&mut self, input: event::MouseEvent) {
        match input {
            event::MouseEvent::Down(button, click_x, click_y, _) | event::MouseEvent::Drag(button, click_x, click_y, _) => {
                let click_x_index = (self.camera.x + (click_x / 2) as usize).checked_sub(self.camera.viewport_width / 2);
                let click_y_index = (self.camera.y + click_y as usize).checked_sub(self.camera.viewport_height / 2);

                let positions = [click_x_index, click_y_index];
                match positions {
                    [Some(click_x_index), Some(click_y_index)] => {
                        if let Some(index) = self.map.pos(click_x_index, click_y_index) {
                            match button {
                                event::MouseButton::Left => self.map.cells[index] = true,
                                event::MouseButton::Right => self.map.cells[index] = false,
                                _ => (),

                            }

                        }
                    },
                    _ => ()
                }
            },
            _ => (),
        }

        self.render_status();
        self.render_map();
    }

    pub fn process_key_input(&mut self, input: event::KeyCode) {
        match input {
            event::KeyCode::Char('q') => self.running = false,
            event::KeyCode::Char('w') | event::KeyCode::Char('k') => self.camera.pan(Direction::North, self.map.width, self.map.height),
            event::KeyCode::Char('a') | event::KeyCode::Char('h') => self.camera.pan(Direction::West, self.map.width, self.map.height),
            event::KeyCode::Char('s') | event::KeyCode::Char('j') => self.camera.pan(Direction::South, self.map.width, self.map.height),
            event::KeyCode::Char('d') | event::KeyCode::Char('l') => self.camera.pan(Direction::East, self.map.width, self.map.height),
            event::KeyCode::Char('c') => { self.map.clear_map(); self.update_count = 0; },
            event::KeyCode::Char('+') => self.camera.speed = util::clamp(self.camera.speed + 1, 1, min(self.camera.viewport_width, self.camera.viewport_height)),
            event::KeyCode::Char('-') => self.camera.speed = util::clamp(self.camera.speed - 1, 1, min(self.camera.viewport_width, self.camera.viewport_height)),
            event::KeyCode::Char('r') => self.map.randomize_map(),
            event::KeyCode::Char(' ') => self.paused = !self.paused,
            _ => (),
        }
    }

    pub fn resize_viewport(&mut self, width: usize, height: usize) {
        self.stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
        self.camera.viewport_width = width / 2;
        self.camera.viewport_height = height;
    }
}
