#[derive(Debug)]
pub struct ChessPosition((i32, i32));

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if !(0..8).contains(&rank) || !(0..8).contains(&file) {
            return None;
        }

        Some(ChessPosition((rank, file)))
    }

    pub fn rank(&self) -> i32 {
        self.0 .0
    }
    pub fn file(&self) -> i32 {
        self.0 .1
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen(position)
    }

    pub fn rank(&self) -> i32 {
        self.0.rank()
    }

    pub fn file(&self) -> i32 {
        self.0.file()
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.rank() == other.rank() || self.file() == other.file() || self.is_diagonal(other)
    }

    pub fn is_diagonal(&self, other: &Queen) -> bool {
        let rank_d = self.rank() - other.rank();
        let file_d = self.file() - other.file();

        rank_d.abs() == file_d.abs()
    }
}
