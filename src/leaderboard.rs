use std::{fs::File, path::Path, io::{BufReader, BufRead}};

use crate::leaderboard_entry::LeaderboardEntry;

pub struct Leaderboard {
    file: File,
    leaderboard: Vec<LeaderboardEntry>
}

impl Leaderboard {
    pub fn new() -> Leaderboard {
        let leaderboard_file: File = get_or_create_leaderboard_file();

        Leaderboard {
            leaderboard: read_leaderboard(&leaderboard_file),
            file: leaderboard_file,
        }
    }
}

pub fn get_or_create_leaderboard_file() -> File {
    // Create a path to the desired file
    let path: &Path = Path::new("target/leaderboard.txt");

    // Open the path in read-only mode, returns `io::Result<File>`
    let file: File = match File::open(&path) {
        Ok(file) => file,
        Err(..) => {
            File::create("target/leaderboard.txt")
                .expect("Unable to create file, probably due to lack of permissions in your Operative System")
        }
    };

    file
}

pub fn read_leaderboard(file: &File) -> Vec<LeaderboardEntry> {
    let mut leaderboard: Vec<LeaderboardEntry> = Vec::with_capacity(10);

    let reader: BufReader<&File> = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(value) => leaderboard.insert(index, LeaderboardEntry::new_from_file_line(value)),
            Err(err)   => panic!("Something went wrong reading leaderboard!\n{}", err)
        }
    }

    leaderboard
}