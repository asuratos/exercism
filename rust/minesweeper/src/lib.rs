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

    pub fn from_rowlist(rows: &[&str]) -> Self{
        Board {
            dims: (rows.len(), rows[1].len()),
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

    fn get_neighbors_of(&self, i: usize) -> [usize] {
        let coords = self.address_to_xy(i);
        let w = self.dims.0;
        let h = self.dims.1;

        if coords == (0, 0) {
            return [0, 1, self.dims.0, self.dims.0 + 1];
        } else {
            return [1];
        }
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let board = Board::from_rowlist(minefield);
    unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{:#?}\n", minefield);
}
