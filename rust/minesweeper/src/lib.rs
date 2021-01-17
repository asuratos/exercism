use std::str;

pub struct Board<'a> {
    tiles: &'a [&'a str],
    dims: (usize, usize),
}

impl<'a> Board<'a> {
    pub fn from_rowlist(rows: &'a [&'a str]) -> Self {
        if rows.is_empty() {
            Board {
                dims: (0, 0),
                tiles: rows,
            }
        } else {
            Board {
                dims: (rows[0].len(), rows.len()),
                tiles: rows,
            }
        }
    }

    // internal helpers
    // address transform
    fn address_to_xy(&self, i: usize) -> (usize, usize) {
        (i % self.dims.0, i / self.dims.0)
    }

    pub fn get_neighbors_of(&self, i: usize) -> Vec<(usize, usize)> {
        // returns the neighbors of an indexed tile,
        // in a vector of x,y coordinates
        // lots of literal edge cases here...

        if self.dims == (0, 0) {
            return vec![];
        }
        match self.address_to_xy(i) {
            // corners
            (0, 0) => {
                //upper left
                if self.dims.1 == 1 {
                    // if only one row
                    return vec![(1, 0)];
                } else if self.dims.0 == 1 {
                    // if only one column
                    return vec![(0, 1)];
                } else {
                    return vec![(0, 1), (1, 0), (1, 1)];
                }
            }
            (w, 0) if (w == self.dims.0 - 1) => {
                // upper right
                if self.dims.1 == 1 {
                    // if one row
                    return vec![(w - 1, 0)];
                } else {
                    return vec![(w - 1, 0), (w, 1), (w - 1, 1)];
                }
            }
            (0, h) if (h == self.dims.1 - 1) => {
                // lower left
                if self.dims.0 == 1 {
                    // if one column
                    return vec![(0, h - 1)];
                } else {
                    return vec![(1, h), (0, h - 1), (1, h - 1)];
                }
            }
            (w, h) if (h == self.dims.1 - 1) && (w == self.dims.0 - 1) => {
                // lower right
                return vec![(w - 1, h), (w, h - 1), (w - 1, h - 1)];
            }
            // walls
            (w, 0) => {
                // first row
                if self.dims.1 == 1 {
                    // only 1 row
                    return vec![(w - 1, 0), (w + 1, 0)];
                } else {
                    return vec![(w - 1, 0), (w + 1, 0), (w - 1, 1), (w, 1), (w + 1, 1)];
                }
            }
            (0, h) => {
                // first column
                if self.dims.0 == 1 {
                    // only 1 column
                    return vec![(0, h + 1), (0, h - 1)];
                } else {
                    return vec![(0, h + 1), (0, h - 1), (1, h + 1), (1, h), (1, h - 1)];
                }
            }
            (w, h) if h == self.dims.1 - 1 => {
                // last row
                if self.dims.1 == 1 {
                    // only 1 row
                    return vec![(w - 1, 0), (w + 1, 0)];
                } else {
                    return vec![
                        (w - 1, h),
                        (w + 1, h),
                        (w - 1, h - 1),
                        (w, h - 1),
                        (w + 1, h - 1),
                    ];
                }
            }
            (w, h) if w == self.dims.0 - 1 => {
                // last column
                if self.dims.0 == 1 {
                    // only 1 column
                    return vec![(0, h + 1), (0, h - 1)];
                } else {
                    return vec![
                        (w, h + 1),
                        (w, h - 1),
                        (w - 1, h + 1),
                        (w - 1, h),
                        (w - 1, h - 1),
                    ];
                }
            }
            (w, h) => {
                // anywhere in the middle
                return vec![
                    (w, h + 1),
                    (w, h - 1),
                    (w - 1, h - 1),
                    (w - 1, h),
                    (w - 1, h + 1),
                    (w + 1, h - 1),
                    (w + 1, h),
                    (w + 1, h + 1),
                ];
            }
        }
    }

    fn count_neighboring_mines(&self, i: usize) -> String {
        match self
            .get_neighbors_of(i)
            .iter()
            .filter(|&&(x, y)| self.tiles[y].chars().nth(x).unwrap() == '*')
            .count()
        {
            0 => " ".to_string(),
            i => i.to_string(),
        }
    }

    fn annotate(&self) -> Vec<String> {
        self.tiles
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        '*' => "*".to_string(),
                        _ => self.count_neighboring_mines((i * self.dims.0) + j),
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let board = Board::from_rowlist(minefield);
    board.annotate()
}
