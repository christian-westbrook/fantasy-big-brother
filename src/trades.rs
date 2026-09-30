use std::collections::HashMap;
use std::io::BufRead;

pub fn get_trades(trades_reader: impl BufRead) -> Result<HashMap<String, Vec<(String, String, String)>>, String> {

    let mut trades = HashMap::new();

    for line in trades_reader.lines() {
        match line {
            Ok(line) if line.is_empty() => {},
            Ok(line) => {
                let (episode, game, player_one, player_two) = parse_trades_line(&line)
                    .expect(&format!("Unable to parse a line of the input trades text '{}'", line));
                
                if !trades.contains_key(&episode) {
                    trades.insert(episode.clone(), Vec::new());
                }
                
                trades.get_mut(&episode)
                    .expect(&format!("Expected the trades map to already contain the key {}", episode))
                    .push((game, player_one, player_two));
            },
            _ => {},
        }
    }

    Ok(trades)
}

fn parse_trades_line(line: &str) -> Result<(String, String, String, String), String> {
    let mut tokens = line.split(",");

    let episode = tokens.nth(0)
        .expect(&format!("Expected an episode to be parseable from input line {}", line))
        .to_string();

    let game = tokens.nth(0)
        .expect(&format!("Expected a game to be parseable from input line {}", line))
        .to_string();

    let player_one = tokens.nth(0)
        .expect(&format!("Expected a player one to be parseable from input line {}", line))
        .to_string();

    let player_two = tokens.nth(0)
        .expect(&format!("Expected a player one to be parseable from input line {}", line))
        .to_string();

    Ok((episode, game, player_one, player_two))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_trades_parses_trades_correctly() {
        let trades_definition = "5,family,angela,barret";
        let trades = get_trades(trades_definition.as_bytes())
            .expect(&format!("Expected get_trades() to successfully handle input text {}", trades_definition));
        assert_eq!(*trades.get("5").unwrap(), vec![("family".to_string(), "angela".to_string(), "barret".to_string())])
    }
}