use crossterm::{cursor, style::Print, ExecutableCommand, QueueableCommand};

pub struct Game {
    pub stdout: std::io::Stdout,
    pub map: Map,
    pub running: bool,
}

impl Game {
    pub fn new(stdout: std::io::Stdout, map: Map) -> Self {
        Game {
            stdout,
            map,
            running: false,
        }
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

pub struct Map {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Map {
            width,
            height,
            cells: vec![false; width * height],
        }
    }

    pub fn pos(&self, x: usize, y: usize) -> usize {
        (x * self.width) + y
    }
}
