pub struct Board<'a> {
    tiles: &'a [&'a str],
    dims: (usize, usize),
}

impl<'a> Board<'a> {
    // builders
    // pub fn new() -> Self {
    //     Board {
    //         tiles: &[""],
    //         dims: (0, 0),
    //     }
    // }

    pub fn from_rowlist(rows: &'a [&'a str]) -> Self {
        println!("{:?}", rows);
        Board {
            dims: (rows[1].len(), rows.len()),
            tiles: rows,
        }
    }

    // internal helpers
    // address transforms
    fn address_to_xy(&self, i: usize) -> (usize, usize) {
        (i % self.dims.1, i / self.dims.1)
    }

    pub fn get_neighbors_of(&self, i: usize) -> Vec<(usize, usize)> {
        match self.address_to_xy(i) {
            // corners
            (0, 0) => {
                //upper left
                return vec![(0, 1), (1, 0), (1, 1)];
            }
            (w, 0) if (w == self.dims.0 - 1) => {
                // upper right
                return vec![(w - 1, 0), (w, 1), (w - 1, 1)];
            }
            (0, h) if (h == self.dims.1 - 1) => {
                // lower left
                return vec![(1, h), (0, h - 1), (1, h - 1)];
            }
            (w, h) if (h == self.dims.1 - 1) && (w == self.dims.0 - 1) => {
                // lower right
                return vec![(w - 1, h), (w, h - 1), (w - 1, h - 1)];
            }
            // walls
            (w, 0) => {
                // first row
                return vec![(w - 1, 0), (w + 1, 0), (w - 1, 1), (w, 1), (w + 1, 1)];
            }
            (0, h) => {
                // first column
                return vec![(0, h + 1), (0, h - 1), (1, h + 1), (1, h), (1, h - 1)];
            }
            (w, h) if h == self.dims.1 - 1 => {
                // last row
                return vec![
                    (w - 1, h),
                    (w + 1, h),
                    (w - 1, h - 1),
                    (w, h - 1),
                    (w + 1, h - 1),
                ];
            }
            (w, h) if w == self.dims.0 - 1 => {
                // last column
                return vec![
                    (w, h + 1),
                    (w, h - 1),
                    (w - 1, h + 1),
                    (w - 1, h),
                    (w - 1, h - 1),
                ];
            }
            (w, h) => {
                // anywhere in the middle
                return vec![
                    (w, h + 1),
                    (w, h - 1),
                    (w - 1, h),
                    (w + 1, h),
                    (w - 1, h - 1),
                    (w + 1, h - 1),
                    (w - 1, h + 1),
                    (w + 1, h + 1),
                ];
            }
        }
    }

    fn count_neighboring_mines(&self, i: usize) -> char {
        self.get_neighbors_of(i)
            .iter()
            .filter(|&&(x, y)| (self.tiles[y-1..=y-1])[x-1..=x-1] == ["*"])
            .count() as u8 as char
    }

    fn annotate(&self) -> Vec<String> {
        self.tiles.iter().map(|row| {
            row.chars().enumerate().map(|(i, c)| match c {
                '*' => '*',
                _ => self.count_neighboring_mines(i),
            }).collect::<String>()
        }).collect::<Vec<String>>()
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let board = Board::from_rowlist(minefield);
    board.annotate()
}
