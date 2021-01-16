pub struct Board {
    tiles: String,
    dims: (usize, usize),
}

impl Board {
    // builders
    pub fn new() -> Self {
        Board {
            tiles: String::new(),
            dims: (0, 0),
        }
    }

    pub fn from_rowlist(rows: &[&str]) -> Self {
        Board {
            dims: (rows[1].len(), rows.len()),
            tiles: rows.concat(),
        }
    }

    // internal helpers
    // address transforms
    fn xy_to_address(&self, x: usize, y: usize) -> usize {
        (y * self.dims.1) + x
    }

    fn address_to_xy(&self, i: usize) -> (usize, usize) {
        (i % self.dims.1, i / self.dims.1)
    }

    pub fn get_neighbors_of(&self, i: usize) -> Vec<usize> {
        match self.address_to_xy(i) {
            // corners
            (0, 0) => {
                //upper left
                return [(0, 1), (1, 0), (1, 1)]
                    .iter()
                    .map(|&(x, y)| self.xy_to_address(x, y))
                    .collect();
            }
            (w, 0) if (w == self.dims.0 - 1) => {
                // upper right
                return [(w - 1, 0), (w, 1), (w - 1, 1)]
                    .iter()
                    .map(|&(x, y)| self.xy_to_address(x, y))
                    .collect();
            }
            (0, h) if (h == self.dims.1 - 1) => {
                // lower left
                return [(1, h), (0, h - 1), (1, h - 1)]
                    .iter()
                    .map(|&(x, y)| self.xy_to_address(x, y))
                    .collect();
            }
            (w, h) if (h == self.dims.1 - 1) && (w == self.dims.0 - 1) => {
                // lower right
                return [(w - 1, h), (w, h - 1), (w - 1, h - 1)]
                    .iter()
                    .map(|&(x, y)| self.xy_to_address(x, y))
                    .collect();
            }
            // walls
            (w, 0) => {
                // first row
                return [(w - 1, 0), (w + 1, 0), (w - 1, 1), (w, 1), (w + 1, 1)]
                    .iter()
                    .map(|&(x, y)| self.xy_to_address(x, y))
                    .collect();
            }
            (0, h) => {
                // first column
                return [(0, h + 1), (0, h - 1), (1, h + 1), (1, h), (1, h - 1)]
                    .iter()
                    .map(|&(x, y)| self.xy_to_address(x, y))
                    .collect();
            }
            (w, h) if h == self.dims.1 - 1 => {
                // last row
                return [
                    (w - 1, h),
                    (w + 1, h),
                    (w - 1, h - 1),
                    (w, h - 1),
                    (w + 1, h - 1),
                ]
                .iter()
                .map(|&(x, y)| self.xy_to_address(x, y))
                .collect();
            }
            (w, h) if w == self.dims.0 - 1 => {
                // last column
                return [
                    (w, h + 1),
                    (w, h - 1),
                    (w - 1, h + 1),
                    (w - 1, h),
                    (w - 1, h - 1),
                ]
                .iter()
                .map(|&(x, y)| self.xy_to_address(x, y))
                .collect();
            }
            (w, h) => {
                // anywhere in the middle
                return [
                    (w, h + 1),
                    (w, h - 1),
                    (w - 1, h),
                    (w + 1, h),
                    (w - 1, h - 1),
                    (w + 1, h - 1),
                    (w - 1, h + 1),
                    (w + 1, h + 1),
                ]
                .iter()
                .map(|&(x, y)| self.xy_to_address(x, y))
                .collect();
            }
        }
    }

    fn count_neighboring_mines(&self, i: usize) -> u32 {
        self.get_neighbors_of(i)
            .iter()
            .filter(|&&n| self.tiles.chars().nth(n).unwrap() == '*')
            .count() as u32
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let board = Board::from_rowlist(minefield);
    unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{:#?}\n", minefield);
}
