use std::collections::HashMap;
use std::io::BufRead;

use regex::Regex;

/// Accepts a reader representing a textually defined scoring system for
/// a game of fantasy Big Brother and attempts to parse it into a map of 
/// scoring events to scores. Wraps the output into a Result.
///
/// For instance, the line 'hoh_winner = 5' represents a rule awarding 5
/// points to the houseguest that wins a head of household competition. This
/// function would insert 'hoh_winner' as a key into the scoring map with its
/// value set to the 5_i32.
///
/// Inputs
/// path: Path to a scoring file mapping scoring event keys to their values
///
/// Outputs
/// scoring: Result containing a HashMap of scoring event keys to scores 
///
/// Examples
///
/// ```
/// use fantasy_big_brother::scoring::get_scoring;
///
/// let input = "test_event = 3";
/// let scoring = get_scoring(input.as_bytes()).unwrap();
/// assert_eq!(*scoring.get("test_event").unwrap(), 3);
/// ```
///
pub fn get_scoring(scoring_reader: impl BufRead) -> Result<HashMap<String, i32>, String> {

    let mut scoring = HashMap::new();

    let regex = r"[a-z_]+\s*=\s*-?\d+\s*#?";
    let regex = Regex::new(regex).unwrap();
    for line in scoring_reader.lines() {
        match line {
            Ok(line) if line.is_empty() => {},
            Ok(line) if regex.is_match(&line) => { 
                let event_key = line.split("=").nth(0)
                    .expect(&format!("Expected an event key to be present for the line '{}' that passed the regex filter '{}'", line, regex.as_str()))
                    .trim();

                match get_score_value_from_line(&line) {
                    Ok(score) => { scoring.insert(event_key.to_string(), score); },
                    Err(err) => { return Err(err); },
                };
             },
            _ => {},
        }
    }

    Ok(scoring)
}

pub fn get_results(roster: &Vec<String>, scoring: &HashMap<String, i32>, results_reader: impl BufRead) -> Result<HashMap<String, i32>, String> {
    let mut results: HashMap<String, i32> = roster.iter().map(|houseguest| (houseguest.clone(), 0)).collect();

    let regex = r"[0-9]+,[a-z]+,[a-z_]+";
    let regex = Regex::new(regex).unwrap();
    for line in results_reader.lines() {
        match line {
            Ok(line) if line.is_empty() => {},
            Ok(line) if regex.is_match(&line) => {
                match parse_results_line(&line) {
                    Ok((_episode, houseguest, scoring_event)) => { 
                        let Some(current_score) = results.get(&houseguest) else {
                            return Err(format!("Expected houseguest '{}' to be present in results map {:?}", houseguest, results));
                        };

                        let Some(scoring_event_value) = scoring.get(&scoring_event) else {
                            return Err(format!("Expected scoring event '{}' to be present in scoring system map\n\n{:?}", scoring_event, scoring));
                        };

                        results.insert(houseguest, scoring_event_value + current_score);
                    },
                    Err(err) => { return Err(err); },
                }
            },
            _ => {},
        }
    }

    Ok(results)
}

/// Accepts a single line of text representing a scoring event within a scoring
/// system for fantasy big brother and attempts to parse the score into an i32.
fn get_score_value_from_line(line: &str) -> Result<i32, String> {
    let score_str = line.split("#").nth(0).ok_or(format!("Expected input line '{}' to have a value preceding any # character", line))?
        .split("=").nth(1).ok_or(format!("Expected input line '{}' to have a value following an = character", line))?
        .trim();

    match score_str.parse() {
        Ok(score) => Ok(score),
        _ => Err(format!("Expected the retrieved score value '{}' to be parseable into an i32", score_str))
    }
}

/// Accepts a single line of text representing a scoring event within a stream
/// representing the results of a game of fantasy Big Brother and attempts to
/// parse the episode, houseguest, and event.
fn parse_results_line(line: &str) -> Result<(String, String, String), String> {
    let mut tokens = line.split(",");

    let Some(episode) = tokens.nth(0) else {
        return Err(format!("Expected a episode to be parseable from the input line '{}' split into tokens '{:?}'", line, tokens));
    };

    let Some(houseguest) = tokens.nth(0) else {
        return Err(format!("Expected a houseguest to be parseable from the input line '{}' split into tokens '{:?}'", line, tokens));
    };

    let Some(scoring_event) = tokens.nth(0) else {
        return Err(format!("Expected a scoring event to be parseable from the input line '{}' split into tokens '{:?}'", line, tokens));
    };

    Ok((episode.to_string(), houseguest.to_string(), scoring_event.to_string()))
}

#[cfg(test)]
mod tests {

    use super::*;

    use std::io::BufReader;

    use rstest::rstest;
    use proptest::prelude::*;

    #[rstest]
    #[case("\ntest_event = 3\n\n", "test_event", 3)]
    #[case("blergh=99\n\nblargh=27\n", "blargh", 27)]
    #[case("blergh=99\n\nblargh=27\n", "blergh", 99)]
    fn get_scoring_parses_scoring_correctly(
        #[case] input: &str,
        #[case] event_key: &str,
        #[case] expected: i32
    ) {
        let scoring: HashMap<String, i32> = get_scoring(input.as_bytes())
            .expect(&format!("Expected get_scoring() to return Ok() for input '{}'", input));
        let actual = *scoring.get(event_key)
            .expect(&format!("Expected the event key '{}' to be present in the scoring map\n\n{:?}", event_key, scoring));

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case("\ntest_event = 3\n\n", "test_event", 3)]
    #[case("blergh=99\n\nblargh=27\n", "blargh", 27)]
    #[case("blergh=99\n\nblargh=27\n", "blergh", 99)]
    fn get_scoring_accepts_buf_readers(
        #[case] input: &str,
        #[case] event_key: &str,
        #[case] expected: i32
    ) {
        let reader = BufReader::new(input.as_bytes());
        let scoring: HashMap<String, i32> = get_scoring(reader)
            .expect(&format!("Expected get_scoring() to return Ok() for input '{:?}'", input));
        let actual = *scoring.get(event_key)
            .expect(&format!("Expected the event key '{}' to be present in the scoring map\n\n{:?}", event_key, scoring));

        assert_eq!(expected, actual);
    }

    #[test]
    fn get_results_parses_scores_correctly() {
        let scoring_system_definition = "\ntest_win=3\ntest_loss=-1";
        let scoring = get_scoring(scoring_system_definition.as_bytes())
            .expect(&format!("Expected get_scoring() to successfully handle input text '{}'", scoring_system_definition));

        let roster_definition = "alice\nbob";
        let roster = crate::roster::get_roster(roster_definition.as_bytes())
            .expect(&format!("Expected get_roster() to successfully handle input text '{}", roster_definition));

        let results_definition = "1,alice,test_win\n1,bob,test_loss\n2,bob,test_win\n3,bob,test_loss";
        let results = get_results(&roster, &scoring, results_definition.as_bytes())
            .expect(&format!("Expected get_results() to successfully handle input roster '{:?}' scoring system '{:?}' and results text '{}'", roster, scoring, results_definition));
        
        assert_eq!(results, HashMap::from([("alice".to_string(), 3), ("bob".to_string(), 1)]));
    }

    proptest! {
        #[test]
        fn get_scoring_doesnt_crash(s in "\\PC*") {
            let _ = get_scoring(s.as_bytes());
        }

        #[test]
        fn get_scoring_parses_scoring_correctly_proptest(
            event_key in "[a-z_]+",
            score in 0i32..i32::MAX
        ) {
            let input = format!("{} = {}", event_key, score);

            let scoring: HashMap<String, i32> = get_scoring(input.as_bytes())
                .expect(&format!("Expected get_scoring() to return Ok() for input '{}'", input));
            let actual = *scoring.get(&event_key)
                .expect(&format!("Expected the event key '{}' to be present in the scoring map: {:?}", event_key, scoring));
            let expected = score;

            assert_eq!(expected, actual);
        }
    }
}
