use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_roster() -> Vec<String> {

    let mut roster = Vec::new();

    let roster_file = File::open("data/roster.dat")
        .expect("Expected the roster.dat file to be present");
    let roster_lines = BufReader::new(roster_file).lines();
    for line in roster_lines {
        match line {
            Ok(line) => roster.push(line),
            _ => {},
        }
    }

    roster
}