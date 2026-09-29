use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_roster(reader: impl BufRead) -> Result<Vec<String>, String> {

    let mut roster = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line) if line.is_empty() => {},
            Ok(line) => { roster.push(line.trim().to_string()); },
            _ => {},
        }
    }

    Ok(roster)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_roster_loads_correct_roster() {
        let roster_definition = "alice\nbob";
        let roster = get_roster(roster_definition.as_bytes())
            .expect(&format!("Expected get_roster() to succeed at parsing the input string '{}'", roster_definition));
        assert_eq!(roster, vec!["alice", "bob"]);
    }
}