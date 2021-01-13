#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: Vec::from(scores),
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        match self.scores.len() {
            0 => None,
            _ => Some(self.scores[self.scores.len() - 1]),
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        if let Some(max) = self.scores.iter().max() {
            Some(*max)
        } else {
            None
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted = self.scores.clone();
        sorted.sort_unstable();
        sorted
            .iter()
            .rev()
            .take(3)
            .map(|&c| c)
            .collect::<Vec<u32>>()
    }
}
