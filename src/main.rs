use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use big_brother::scoring::get_scoring;
use big_brother::roster::get_roster;

const SCORING_DATA_PATH: &str = "data/scoring.dat"; 

fn main() {
    let file = File::open(SCORING_DATA_PATH)
        .expect(format!("Expected the file '{}' to be present", SCORING_DATA_PATH).as_str());
    let reader = BufReader::new(file);
    let _scoring = get_scoring(reader);
    
    let roster: Vec<String> = get_roster();

    let mut houseguest_scores = HashMap::new();

    for houseguest in roster {
        houseguest_scores.insert(houseguest, 0);
    }

    println!("{}", houseguest_scores.get("lyric").expect(""));
}

