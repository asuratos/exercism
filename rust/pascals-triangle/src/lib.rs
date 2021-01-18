pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    fn d_level(&self, level: u32) -> Vec<u32> {
        match level {
            0 => vec![],
            1 => vec![1],
            _ => {
                let mut out = self
                    .rows
                    .iter()
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|win| win.iter().sum::<u32>())
                    .collect::<Vec<u32>>();
                out.insert(0, 1);
                out.push(1);
                out
            }
        }
    }

    fn add_row(&mut self) {
        self.rows.push(self.d_level(self.rows.len() as u32 + 1));
    }

    pub fn new(row_count: u32) -> Self {
        let mut out = PascalsTriangle { rows: Vec::new() };
        for _ in 1..=row_count {
            out.add_row();
        }
        out
    }

    pub fn rows(self) -> Vec<Vec<u32>> {
        self.rows
    }
}
