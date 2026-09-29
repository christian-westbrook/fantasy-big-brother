use std::collections::HashMap;
use std::io::BufRead;

use regex::Regex;

/// path: Path to a scoring file mapping scoring event keys to their values
pub fn get_scoring(reader: impl BufRead) -> Result<HashMap<String, i32>, String> {

    let mut scoring = HashMap::new();

    let regex = r"[a-z_]+\s*=\s*-?\d+\s*#?";
    let regex = Regex::new(regex).unwrap();
    for line in reader.lines() {
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

fn get_score_value_from_line(line: &str) -> Result<i32, String> {
    let score_str = line.split("#").nth(0).ok_or(format!("Expected input line '{}' to have a value preceding any # character", line))?
        .split("=").nth(1).ok_or(format!("Expected input line '{}' to have a value following an = character", line))?
        .trim();

    match score_str.parse() {
        Ok(score) => Ok(score),
        _ => Err(format!("Expected the retrieved score value '{}' to be parseable into an i32", score_str))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use rstest::rstest;
    use proptest::prelude::*;

    #[rstest]
    #[case("\ntest_event = 3\n\n", "test_event", 3)]
    #[case("blergh=99\n\nblargh=27\n", "blargh", 27)]
    #[case("blergh=99\n\nblargh=27\n", "blergh", 99)]
    fn scoring_events_are_loaded_accurately(
        #[case] input: &str,
        #[case] event_key: &str,
        #[case] expected: i32
    ) {
        let scoring: HashMap<String, i32> = get_scoring(input.as_bytes())
            .expect(&format!("Expected get_scoring() to return Ok() for input '{}'", input));
        let actual = *scoring.get(event_key)
            .expect(&format!("Expected the event key '{}' to be present in the scoring map: {:?}", event_key, scoring));

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
            .expect(&format!("Expected the event key '{}' to be present in the scoring map: {:?}", event_key, scoring));

        assert_eq!(expected, actual);
    }

    proptest! {
        #[test]
        fn get_scoring_doesnt_crash(s in "\\PC*") {
            get_scoring(s.as_bytes());
        }

        #[test]
        fn scoring_events_are_loaded_accurately_proptest(
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
