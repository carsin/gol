use crossterm::terminal;

use super::game;
use super::util;

pub struct Camera {
    pub viewport_width: usize,
    pub viewport_height: usize,
    pub x: usize,
    pub y: usize,
    pub speed: usize,
}

impl Camera {
    pub fn new(x: usize, y: usize) -> Self {
        let viewport_dimensions = terminal::size().unwrap();
        // Width is half of terminal because each cell is rendered as 2 characters wide
        let viewport_width = (viewport_dimensions.0 / 2) as usize;
        let viewport_height = viewport_dimensions.1 as usize;

        Self {
            viewport_width,
            viewport_height,
            x,
            y,
            speed: 5,
        }
    }

    pub fn pan(&mut self, dir: game::Direction, max_width: usize, max_height: usize) {
        match dir {
            game::Direction::North => self.y = util::clamp(self.y.checked_sub(self.speed).unwrap_or(0), 0, max_height),
            game::Direction::South => self.y = util::clamp(self.y + self.speed, 0, max_height),
            game::Direction::East => self.x = util::clamp(self.x + self.speed, 0, max_width),
            game::Direction::West => self.x = util::clamp(self.x.checked_sub(self.speed).unwrap_or(0), 0, max_width)
        }
    }
}
