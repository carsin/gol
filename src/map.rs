use rand::random;
use std::cmp::min;

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<bool>,
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

    pub fn update(&mut self) {
        let mut next_generation = self.cells.clone();
        for x in 0..self.width {
            for y in 0..self.height {
                let cell_pos = self.pos(x, y).unwrap();
                // Live cell checks
                if self.cells[cell_pos] == true {
                    let next_state = match self.get_cell_live_neighbor_count(x, y) {
                        2 | 3 => true,
                        _ => false,
                    };

                    next_generation[cell_pos] = next_state;
                } else {
                    // Dead cell checks
                    if self.get_cell_live_neighbor_count(x, y) == 3 {
                        next_generation[cell_pos] = true;
                    }
                }
            }
        }

        self.cells = next_generation;
    }

    fn get_cell_live_neighbor_count(&self, cell_x: usize, cell_y: usize) -> usize {
        let mut neighbors: usize = 0;

        let left_neighbors = cell_x.checked_sub(1).unwrap_or(0);
        let top_neighbors = cell_y.checked_sub(1).unwrap_or(0);

        let right_neighbors = min(self.width - 1, cell_x + 1);
        let bottom_neighbors = min(self.height - 1, cell_y + 1);

        for x in left_neighbors..right_neighbors + 1 {
            for y in top_neighbors..bottom_neighbors + 1 {
                if cell_x == x && cell_y == y {
                    continue;
                } else {
                    if let Some(pos) = self.pos(x, y) {
                        if self.cells[pos] {
                            neighbors += 1;
                        }
                    }
                }
            }
        }

        neighbors
    }

    pub fn pos(&self, x: usize, y: usize) -> Option<usize> {
        if x > self.width - 1 || y > self.height - 1 {
            None
        } else {
            Some((y * self.width) + x)
        }
    }
}
