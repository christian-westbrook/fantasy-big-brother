use std::collections::{HashMap, HashSet};
use std::io::BufRead;

pub fn get_draft(draft_reader: impl BufRead) -> Result<HashMap<String, HashMap<String, HashSet<String>>>, String> {

    let mut game_map = HashMap::new();

    for line in draft_reader.lines() {
        match line {
            Ok(line) if line.is_empty() => {},
            Ok(line) => { 
                let Ok((game, team, houseguest)) = parse_draft_line(&line) else {
                    return Err(format!("Unable to parse a line of the input draft text {}", line));
                };

                if !game_map.contains_key(&game) {
                    game_map.insert(game.clone(), HashMap::new());
                }

                let team_map = game_map.get_mut(&game).expect(&format!("Expected the game map to contain the game {}", game));
                if !team_map.contains_key(&team) {
                    team_map.insert(team.clone(), HashSet::new());
                }

                let team_set = team_map.get_mut(&team).expect(&format!("Expected the team map to contain the team {}", team));
                team_set.insert(houseguest);
            },
            _ => {},
        }
    }

    
    Ok(game_map)
}

fn parse_draft_line(line: &str) -> Result<(String, String, String), String> {
    let mut tokens = line.split(",");

    let game = tokens.nth(0)
        .expect(&format!("Expected a game to be parseable from input line '{}'", line))
        .to_string();
    
    let team = tokens.nth(0)
        .expect(&format!("Expected a team to be parseable from input line '{}'", line))
        .to_string();
    
    let houseguest = tokens.nth(0)
        .expect(&format!("Expected a game to be parseable from input line '{}'", line))
        .to_string();

    Ok((game, team, houseguest))
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("family,alice,kamu\nfamily,bob,chuk\nwork,charles,drew\nwork,david,melody", "family", "alice", "kamu")]
    #[case("family,alice,dee\nfamily,bob,chuk\nwork,charles,drew\nwork,david,melody", "family", "alice", "dee")]
    #[case("family,alice,dee\nfamily,bob,chuk\nwork,charles,drew\nwork,david,melody", "work", "david", "melody")]
    fn get_draft_parses_correct_teams(
        #[case] draft_definition: &str, 
        #[case] game: &str, 
        #[case] team: &str, 
        #[case] houseguest: &str
    ) {
        let draft = get_draft(draft_definition.as_bytes()).expect(&format!("Expected get_draft() to successfully handle input text '{}'", draft_definition));
        assert!(draft.get(game).unwrap().get(team).unwrap().contains(houseguest));
    }
}