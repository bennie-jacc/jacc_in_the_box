use std::cmp::Ordering;

pub struct LeaderboardEntry {
    pub name: String,
    pub highscore: f32
}

impl LeaderboardEntry {
    pub fn new(name: String, highscore: f32) -> LeaderboardEntry {
        LeaderboardEntry { name , highscore }
    }

    pub fn new_from_file(line: String) -> LeaderboardEntry {
        let split: Vec<&str> = line.split(';').collect();

        LeaderboardEntry { 
            name: String::from(split[0]),
            highscore: split[1].parse::<f32>().expect("A corrupted highscore was found at the leaderboard text file. Unable to parse value to F32")
        }
    }
}

impl Ord for LeaderboardEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.highscore > other.highscore         { Ordering::Greater }
        else if self.highscore < other.highscore    { Ordering::Less }
        else                                        { Ordering::Equal }
    }
}

impl PartialOrd for LeaderboardEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for LeaderboardEntry {}

impl PartialEq for LeaderboardEntry {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}