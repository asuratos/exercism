// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    position: (i32, i32),
    facing: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            position: (x, y),
            facing: d,
        }
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        self.facing = match &self.facing {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };

        self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        self.facing = match &self.facing {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };

        self
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        let d = match self.direction() {
            Direction::North => (0, 1),
            Direction::East => (1, 0),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
        };

        self.position.0 += d.0;
        self.position.1 += d.1;
        self
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |acc, c| match c {
            'L' => acc.turn_left(),
            'R' => acc.turn_right(),
            'A' => acc.advance(),
            _ => acc,
        })
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.facing
    }
}
