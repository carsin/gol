use rand::random;
//use std::cmp::min;

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

    fn get_cell_live_neighbor_count(&self, x: usize, y: usize) -> usize {
        /* Neighbor points diagram {{{
        *                 |         |
        *         x-1,y-1 | x,  y-1 | x+1,y-1
        *                 |         |
        *         --------+---------+----------
        *                 |         |
        *         x-1,y   | x,  y   | x+1,y
        *                 |         |
        *         --------+---------+----------
        *                 |         |
        *         x-1,y+1 | x,  y+1 | x+1,y+1
        *                 |         |
        *
        }}}*/
        let mut neighbors: usize = 0;
        // Neighbor checks {{{
        // Top row checks
        // If not on top edge
        if y > 0 {
            // If not on left edge
            if x > 0 {
                neighbors += self.get_state_at_pos(x - 1, y - 1).unwrap_or(false) as usize; // Top left
            }

            neighbors += self.get_state_at_pos(x, y - 1).unwrap_or(false) as usize; // Top middle

            // If not on right edge
            if x < self.width - 1 {
                neighbors += self.get_state_at_pos(x + 1, y - 1).unwrap_or(false) as usize; // Top right
            }
        }

        // Middle row checks
        // If not on left edge
        if x > 0 {
            neighbors += self.get_state_at_pos(x - 1, y).unwrap_or(false) as usize; // Middle left
        }

        // If not on right edge
        if x < self.width - 1 {
            neighbors += self.get_state_at_pos(x + 1, y).unwrap_or(false) as usize; // Middle right
        }

        // Bottom row checks
        // If not on bottom edge
        if y < self.height - 1 {
            // If not on left edge
            if x > 0 {
                neighbors += self.get_state_at_pos(x - 1, y + 1).unwrap_or(false) as usize; // Bottom left
            }

            neighbors += self.get_state_at_pos(x, y + 1).unwrap_or(false) as usize; // Bottom middle

            // If not on right edge
            if x < self.width - 1 {
                neighbors += self.get_state_at_pos(x + 1, y + 1).unwrap_or(false) as usize; // Bottom right
            }
        }
        // }}}
        neighbors
    }

    pub fn change_cell_state(&mut self, x: usize, y: usize) {
        if let Some(pos) = self.pos(x, y) {
            self.cells[pos] = !self.cells[pos];
        }
    }

    fn get_state_at_pos(&self, x: usize, y: usize) -> Option<bool> {
        let index = self.pos(x, y);
        if let Some(pos) = index {
            Some(self.cells[pos])
        } else {
            None
        }
    }

    pub fn pos(&self, x: usize, y: usize) -> Option<usize> {
        // Only have to check if beyond array on positive side as input parameters are unsigned
        if x > self.width - 1 || y > self.height - 1 {
            None
        } else {
            Some((y * self.width) + x)
        }
    }
}
