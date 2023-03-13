use std::{fs::File, path::Path, io::{BufReader, BufRead, Write}};

use crate::leaderboard_entry::LeaderboardEntry;

pub struct Leaderboard {
    file_path: String,
    pub leaderboard: Vec<LeaderboardEntry>
}

impl Leaderboard {
    pub fn new() -> Leaderboard {
        let path: String = String::from("target/leaderboard.txt");
        let leaderboard_file: File = gen_leaderboard(&path);

        Leaderboard {
            leaderboard: process_leaderboard(&leaderboard_file),
            file_path: String::from(path),
        }
    }

    pub fn update_highscores(&mut self, entry: LeaderboardEntry) -> bool {
        let pos = self.leaderboard.binary_search(&entry).unwrap_or_else(|e| e);
        
        // If position found is within the first 10 highscores: insert at position; shift others to the right.
        // Also pop if list grew further than 10 which is our max. list size
        if pos < 10 { 
            self.leaderboard.insert(pos, entry);
            if self.leaderboard.len() > 10 { self.leaderboard.pop(); }

            self.update_file();

            return true;
        }

        false
    }

    fn update_file(&mut self) {
        let mut file: File = File::create(&self.file_path)
            .expect("Attempted to regenerate and/or truncate file and something went wrong.");

        // Format a text string to then write it all at once.
        let mut contents: String = String::new();
        
        for entry in &self.leaderboard {
            contents.push_str(
                &format!("{};{}\r\n", entry.name, entry.highscore)
            )
        }

        file.write_all(contents.as_bytes()).expect("Unable to write to file (attempting to update it)");
    }
}

fn gen_leaderboard(path_str: &String) -> File {
    // Create a path to the desired file
    let path: &Path = Path::new(path_str);

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

fn process_leaderboard(file: &File) -> Vec<LeaderboardEntry> {
    let mut leaderboard: Vec<LeaderboardEntry> = Vec::with_capacity(10);
    let reader: BufReader<&File> = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(value)   => leaderboard.push(LeaderboardEntry::new_from_file(value)),
            Err(..)             => continue
        }
    }

    leaderboard
}