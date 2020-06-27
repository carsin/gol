use crossterm::{cursor, style::Print, ExecutableCommand, QueueableCommand};
use rand::random;

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

pub struct Map {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        // Initialize the map randomly
        let mut cells = vec![false; width * height];
        for i in 0..cells.len() {
            cells[i] = random(); // Random generates a bool
        }

        Map {
            width,
            height,
            cells,
        }
    }

    fn update(&self) {
        let mut new_cells = self.cells.clone();
    }

    fn pos(&self, x: usize, y: usize) -> usize {
        (x * self.width) + y
    }
}
