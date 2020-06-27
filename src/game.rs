pub struct Map {
    width: usize,
    height: usize,
    cells: Vec<bool>, // Switch to array?
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Map {
            width,
            height,
            cells: vec![false; width * height],
        }
    }

    pub fn get_map_string(&self) -> String {
        let mut map_string = String::new();
        for i in 0..self.cells.len() {
            match self.cells[i] {
                false => map_string.push_str(". "),
                true => map_string.push_str("██"),
            }

            // Append newline character if we're at the edge of the map
            if (i + 1) % self.width == 0 {
                map_string.push_str("\n");
            }
        }
        map_string
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        (x * self.width) + y
    }
}
