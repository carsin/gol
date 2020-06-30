use crossterm::{cursor, style::Print, ExecutableCommand, QueueableCommand};
use super::map;

pub struct Game {
    pub stdout: std::io::Stdout,
    pub map: map::Map,
    pub running: bool,
}

impl Game {
    pub fn new(stdout: std::io::Stdout, map: map::Map) -> Self {
        Game {
            stdout,
            map,
            running: false,
        }
    }

    pub fn update(&mut self) {
        self.map.update();
    }

    pub fn render_map(&mut self) {
        for x in 0..self.map.width {
            for y in 0..self.map.height {
                let chars_to_print = match self.map.cells[self.map.pos(x, y)] {
                    false => ". ",
                    true => "██",
                };

                self.stdout
                    .queue(cursor::MoveTo((x * 2) as u16, y as u16))
                    .unwrap()
                    .execute(Print(chars_to_print))
                    .unwrap();
            }
        }
    }
}
