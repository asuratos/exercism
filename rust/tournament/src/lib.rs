use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {
    let mut board = Board::new();

    if match_results != "" {
        match_results.split("\n").for_each(|line| {
            let result = line.split(";").collect::<Vec<&str>>();

            board.add_team(&result[0]);
            board.add_team(&result[1]);

            match result[2] {
                "win" => board.add_win(result[0], result[1]),
                "loss" => board.add_win(result[1], result[0]),
                "draw" => board.add_draw(result[0], result[1]),
                _ => (),
            }
        });
    }

    // build final table
    // header
    let mut matchres = vec![format!(
        "{:30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
        "Team", "MP", "W", "D", "L", "P"
    )];

    let mut resultsvec = board.all_results();
    resultsvec.sort();

    matchres.extend(resultsvec.iter().rev().map(|line| line.output()));

    matchres.join("\n")
}

struct Board(HashMap<String, TeamRecord>);

impl Board {
    pub fn new() -> Self {
        Board(HashMap::<String, TeamRecord>::new())
    }

    pub fn add_team(&mut self, name: &str) {
        self.0
            .entry(name.to_string())
            .or_insert(TeamRecord::new(name));
    }

    pub fn add_win(&mut self, winner: &str, loser: &str) {
        self.0
            .entry(winner.to_string())
            .and_modify(|team| team.win());
        self.0
            .entry(loser.to_string())
            .and_modify(|team| team.lose());
    }

    pub fn add_draw(&mut self, team1: &str, team2: &str) {
        self.0
            .entry(team1.to_string())
            .and_modify(|team| team.draw());
        self.0
            .entry(team2.to_string())
            .and_modify(|team| team.draw());
    }

    pub fn all_results(&self) -> Vec<&TeamRecord> {
        self.0.values().collect()
    }
}

struct TeamRecord {
    name: String,
    wins: u16,
    losses: u16,
    draws: u16,
}

impl TeamRecord {
    pub fn new(name: &str) -> Self {
        TeamRecord {
            name: name.to_string(),
            wins: 0,
            losses: 0,
            draws: 0,
        }
    }

    pub fn win(&mut self) {
        self.wins += 1;
    }

    pub fn lose(&mut self) {
        self.losses += 1;
    }

    pub fn draw(&mut self) {
        self.draws += 1;
    }

    pub fn output(&self) -> String {
        format!(
            "{:30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            self.name,
            self.matches(),
            self.wins,
            self.draws,
            self.losses,
            self.points()
        )
    }

    fn matches(&self) -> u16 {
        self.wins + self.losses + self.draws
    }

    fn points(&self) -> u16 {
        (self.wins * 3) + self.draws
    }
}

impl PartialEq for TeamRecord {
    fn eq(&self, other: &Self) -> bool {
        self.points() == other.points()
    }
}

impl PartialOrd for TeamRecord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for TeamRecord {}

impl Ord for TeamRecord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.points().cmp(&other.points()) {
            // if points are equal, sort by name
            std::cmp::Ordering::Equal => other.name.cmp(&self.name),
            // otherwise, keep the comparison
            ne => ne,
        }
    }
}
