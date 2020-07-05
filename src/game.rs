use crossterm::{cursor, event, style::Print, terminal, QueueableCommand, ExecutableCommand};

use super::map;

enum Direction {
    North,
    South,
    East,
    West,
}

pub struct Game {
    pub stdout: std::io::Stdout,
    pub map: map::Map,
    pub running: bool,
    pub paused: bool,
    pub viewport_width: usize,
    pub viewport_height: usize,
    pub camera_x: usize,
    pub camera_y: usize,
}

impl Game {
    pub fn new(stdout: std::io::Stdout, map: map::Map) -> Self {
        let viewport_dimensions = terminal::size().unwrap();
        // Width is half of terminal because each cell is rendered as 2 characters wide
        let viewport_width = (viewport_dimensions.0 / 2) as usize;
        let viewport_height = viewport_dimensions.1 as usize;

        let camera_x = map.width / 2;
        let camera_y = map.height / 2;

        Game {
            stdout,
            map,
            running: false,
            paused: false,
            viewport_width,
            viewport_height,
            camera_x,
            camera_y,
        }
    }

    pub fn update(&mut self) {
        self.map.update();
    }

    pub fn render_map(&mut self) {
        for x in 0..self.viewport_width {
            for y in 1..self.viewport_height {
                // Calculate the position to render, None if negative (can't access negative array indicies)
                let x_pos = (x + self.camera_x).checked_sub((self.viewport_width / 2) as usize);
                let y_pos = (y + self.camera_y).checked_sub((self.viewport_height / 2) as usize);
                let positions = [x_pos, y_pos];

                let chars_to_print = match positions {
                    // Good input
                    [Some(x), Some(y)] => {
                        // Check if position is valid (within array)
                        if let Some(pos) = self.map.pos(x, y) {
                            match self.map.cells[pos] {
                                true => "██",
                                false => ". ",
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
            .queue(Print(format!("X: {}, Y: {}", self.camera_x, self.camera_y)))
            .unwrap();
        if self.paused {
            print!(" PAUSED");
        } else {
            print!("       ");
        }
    }

    pub fn process_mouse_input(&mut self, input: event::MouseEvent) {
        match input {
            event::MouseEvent::Down(button, click_x, click_y, _) | event::MouseEvent::Drag(button, click_x, click_y, _) => {
                let click_x_index = (self.camera_x + (click_x / 2) as usize).checked_sub(self.viewport_width / 2);
                let click_y_index = (self.camera_y + click_y as usize).checked_sub(self.viewport_height / 2);

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

        self.render_map();
    }

    pub fn process_key_input(&mut self, input: event::KeyCode) {
        match input {
            event::KeyCode::Char('q') => self.running = false,
            event::KeyCode::Char('w') | event::KeyCode::Char('k') => self.move_camera(Direction::North),
            event::KeyCode::Char('a') | event::KeyCode::Char('h') => self.move_camera(Direction::West),
            event::KeyCode::Char('s') | event::KeyCode::Char('j') => self.move_camera(Direction::South),
            event::KeyCode::Char('d') | event::KeyCode::Char('l') => self.move_camera(Direction::East),
            event::KeyCode::Char('c') => self.map.clear_map(),
            event::KeyCode::Char('r') => self.map.randomize_map(),
            event::KeyCode::Char(' ') => self.paused = !self.paused,
            _ => (),
        }

        self.render_map();
    }

    fn move_camera(&mut self, dir: Direction) {
        match dir {
            Direction::North => {
                if self.camera_y != 0 {
                    self.camera_y -= 1;
                }
            }
            Direction::South => {
                if self.camera_y != self.map.height {
                    self.camera_y += 1;
                }
            }
            Direction::East => {
                if self.camera_x != self.map.width {
                    self.camera_x += 1;
                }
            }
            Direction::West => {
                if self.camera_x != 0 {
                    self.camera_x -= 1;
                }
            }
        }
    }

    // TODO: Fix
    pub fn resize_viewport(&mut self, width: usize, height: usize) {
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();

        self.viewport_width = width / 2;
        self.viewport_height = height;
    }
}
