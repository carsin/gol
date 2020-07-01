use crossterm::{terminal, cursor, style::Print, ExecutableCommand, QueueableCommand};
use super::map;

pub struct Game {
    pub stdout: std::io::Stdout,
    pub map: map::Map,
    pub running: bool,
    pub viewport_width: usize,
    pub viewport_height: usize,
    pub camera_x: usize,
    pub camera_y: usize,
}

impl Game {
    pub fn new(stdout: std::io::Stdout, map: map::Map) -> Self {
        let viewport_dimensions = terminal::size().unwrap();

        // Width is half of terminal because each cell is rendered as 2 characters wide
        let viewport_width = ((viewport_dimensions.0 / 2) as f64).round() as usize;
        let viewport_height = viewport_dimensions.1 as usize;

        let camera_x = map.width / 2;
        let camera_y = map.height / 2;

        println!("width: {}, height: {}", viewport_width, viewport_height);
        Game {
            stdout,
            map,
            running: false,
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
            for y in 0..self.viewport_height {
                let x_pos: usize = x + self.camera_x - (((self.viewport_width / 2) as f64).round() as usize);
                let y_pos: usize = y + self.camera_y - (((self.viewport_height / 2) as f64).round() as usize);
                let chars_to_print = match self.map.cells[self.map.pos(x_pos, y_pos)] {
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

    pub fn resize_viewport(&mut self, width: usize, height: usize) {
        self.viewport_width = width;
        self.viewport_height = height;
        self.render_map();
    }
}
