pub struct LeaderboardEntry {
    name: String,
    score: f32 
}

impl LeaderboardEntry {
    pub fn new(name: String, score: f32) -> LeaderboardEntry {
        LeaderboardEntry {
            name,
            score
        }
    }

    pub fn new_from_file_line(file_line: String) -> LeaderboardEntry {
        let pair: Vec<&str> = file_line.split(';').collect();

        LeaderboardEntry {
            name: String::from(pair[0]),
            score: pair[1].parse::<f32>().expect("Unable to read highscore in file. File is probably corrupted")
        }
    }
}