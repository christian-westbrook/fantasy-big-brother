use std::fs::File;
use std::io::BufReader;

use fantasy_big_brother::roster::get_roster;
use fantasy_big_brother::scoring::get_scoring;
use fantasy_big_brother::scoring::get_results;

const ROSTER_DATA_PATH: &str = "data/roster.dat";
const SCORING_DATA_PATH: &str = "data/scoring.dat";
const RESULTS_DATA_PATH: &str = "data/results.dat"; 

fn main() {
    let roster_reader = get_buf_reader(ROSTER_DATA_PATH);
    let roster = get_roster(roster_reader)
        .expect(&format!("Failed to parse a roster from the input roster file {}", ROSTER_DATA_PATH));

    let scoring_reader = get_buf_reader(SCORING_DATA_PATH);
    let scoring = get_scoring(scoring_reader)
        .expect(&format!("Failed to parse a scoring system from the input scoring file {}", SCORING_DATA_PATH));

    let results_reader = get_buf_reader(RESULTS_DATA_PATH);
    let results = get_results(&roster, &scoring, results_reader)
        .unwrap_or_else(|err| {
            panic!("Failed to parse results from the input results file {} given roster\n\n{:?}\n\nand scoring system\n\n{:?}\n\nError: {}", RESULTS_DATA_PATH, roster, scoring, err);
        });

    println!("{}", results.get("dee").unwrap());
}

/// Creates a BufReader for the file at a given path
fn get_buf_reader(path: &str) -> BufReader<File> {
    let file = File::open(path)
        .expect(&format!("Expected the file '{}' to be present", path));
    
        BufReader::new(file)
}