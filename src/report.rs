use std::collections::{HashSet, HashMap};

use titlecase::titlecase;

pub fn get_report(
    roster: &Vec<String>,
    draft: &HashMap<String, HashMap<String, HashSet<String>>>,
    trades: &HashMap<String, Vec<(String, String, String)>>,
    scoring: &HashMap<String, i32>,
    results: &Vec<(String, String, String)>,
) -> Vec<String> {

    let mut report = Vec::new();

    let mut games: Vec<&String> = draft.keys().collect();
    games.sort();

    for game in games {
        report.push(format!("{} Big Brother", titlecase(game)));
        report.push("".to_string());

        // Teams to their season scores
        let mut teams_to_scores: HashMap<String, i32> = HashMap::new();

        // Houseguests
        report.push("Houseguests".to_string());
        report.push("---".to_string());

        let mut houseguests = roster.clone();
        houseguests.sort();

        for houseguest in houseguests {
            report.push(format!("{}", titlecase(&houseguest)));
        }
        report.push("".to_string());

        // Draft
        report.push("Draft".to_string());
        report.push("".to_string());

        let game_map = draft.get(game).expect(&format!("Unable to get the game '{}' from the draft data structure", game));
        let mut teams: Vec<&String> = game_map.keys().collect();
        teams.sort();

        let mut houseguests_to_teams: HashMap<String, String> = HashMap::new();
        for team in teams {
            report.push(format!("{}", titlecase(team)));
            report.push("---".to_string());

            let team_set = game_map.get(team).expect(&format!("Unable to get the set of players drafted for team '{}'", team));
            for houseguest in team_set {
                houseguests_to_teams.insert(houseguest.clone(), team.clone());
                report.push(format!("{}", titlecase(houseguest)));
            }

            teams_to_scores.insert(team.to_string(), 0);

            report.push("".to_string());
        }

        // Results and standings

        let episodes: HashSet<String> = results.clone()
            .into_iter()
            .map(|(episode, houseguest, scoring_event)| { episode })
            .collect();

        let mut episodes: Vec<String> = episodes.into_iter()
            .collect();

        episodes.sort();

        for episode in episodes {
            report.push(format!("Episode {} Results", episode));
            report.push("---".to_string());

            if let Some(episode_trades) = trades.get(&episode) {
                let episode_trades = episode_trades.clone()
                    .into_iter()
                    .filter(|(g, _houseguest_one, _houseguest_two)| { g == game });

                for (_game, houseguest_one, houseguest_two) in episode_trades {

                    match (houseguests_to_teams.clone().get(&houseguest_one), houseguests_to_teams.clone().get(&houseguest_two)) {
                        (Some(team_one), Some(team_two)) => {
                            houseguests_to_teams.insert(houseguest_one.clone(), team_two.clone());
                            houseguests_to_teams.insert(houseguest_two.clone(), team_one.clone());
                        },

                        (Some(team_one), None) => {
                            houseguests_to_teams.remove(&houseguest_one);
                            houseguests_to_teams.insert(houseguest_two.clone(), team_one.clone());
                        },

                        (None, Some(team_two)) => {
                            houseguests_to_teams.remove(&houseguest_two);
                            houseguests_to_teams.insert(houseguest_one.clone(), team_two.clone());
                        },

                        _ => {}
                    }

                    report.push(format!("{} Traded For {}", titlecase(&houseguest_one), titlecase(&houseguest_two)));
                }
            }

            let episode_results: Vec<(String, String, String)> = results.clone()
                .into_iter()
                .filter(|(e, _houseguest, _event)| { *e == episode })
                .collect();

            // Team to episode score total
            let mut episode_scores: HashMap<String, i32> = HashMap::new();
            
            for (_e, houseguest, event) in episode_results {
                let event_value = scoring.get(&event)
                    .expect(&format!("Expected to find event {} in the scoring map", event));

                report.push(format!("{}, {}, {:+}", titlecase(&houseguest), titlecase(&event), event_value));

                if let Some(team) = houseguests_to_teams.get(&houseguest) {
                    if !episode_scores.contains_key(team) {
                        episode_scores.insert(team.to_string(), 0);
                    }

                    let current_score = episode_scores.get(team)
                        .expect(&format!("Expected to find the team '{}' in the episode_scores map", team));
                    episode_scores.insert(team.to_string(), current_score + event_value);
                }
            }

            report.push("".to_string());

            report.push(format!("Episode {} Standings", episode));
            report.push("---".to_string());

            let teams = teams_to_scores.clone();
            for team in teams.keys() {
                if let Some(episode_score) = episode_scores.get(team) {
                    let score = teams_to_scores.get(team)
                        .expect(&format!("Expected to find the team '{}' in the teams_to_scores map", team));
                    teams_to_scores.insert(team.clone(), *score + episode_score);
                }
            }

            let mut standings: Vec<(String, i32)> = teams_to_scores.clone()
                .into_iter()
                .collect();
            standings.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

            for (index, (team, score)) in standings.iter().enumerate() {
                match episode_scores.get(team) {
                    Some(episode_score) => {
                        report.push(format!("{}. {}: {} ({:+})", (index + 1), titlecase(&team), score, episode_score));
                    },
                    None => {
                        report.push(format!("{}. {}: {}", (index + 1), titlecase(&team), score));
                    }
                }
            }

            report.push("".to_string());
        }

        report.push("".to_string());
    }

    for line in report.clone() {
        eprintln!("{}", line);
    }
    eprintln!();

    report
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::draft::get_draft;
    use crate::roster::get_roster;
    use crate::scoring::{get_scoring, get_results};
    use crate::trades::get_trades;

    #[test]
    fn get_report_creates_correct_report() {
        let roster_definition = "angela\nbarret\nchuk";
        let roster = get_roster(roster_definition.as_bytes())
            .expect(&format!("Expected get_roster() to successfully handle input text {}", roster_definition));

        let draft_definition = "family,alice,angela\nfamily,bob,barret\nwork,charles,chuk\nwork,david,barret";
        let draft = get_draft(draft_definition.as_bytes())
            .expect(&format!("Expected get_draft() to successfully handle input text {}", draft_definition));

        let trades_definition = "2,family,angela,barret\n3,family,angela,chuk";
        let trades = get_trades(trades_definition.as_bytes())
            .expect(&format!("Expected get_trades() to successfully handle input text {}", trades_definition));

        let scoring_definition = "winner=3\nloser=-1";
        let scoring = get_scoring(scoring_definition.as_bytes())
            .expect(&format!("Expected get_scoring() to successfully handle input text {}", scoring_definition));

        let results_definition = "1,angela,winner\n1,chuk,loser\n2,angela,winner\n2,barret,loser\n3,barret,winner\n3,chuk,loser";
        let results = get_results(results_definition.as_bytes())
            .expect(&format!("Expected get_results() to successfully handle input text {}", results_definition));

        let report = get_report(&roster, &draft, &trades, &scoring, &results);

        assert_eq!(
            report,
            vec![
                "Family Big Brother",
                "",
                "Houseguests",
                "---",
                "Angela",
                "Barret",
                "Chuk",
                "",
                "Draft",
                "",
                "Alice",
                "---",
                "Angela",
                "",
                "Bob",
                "---",
                "Barret",
                "",
                "Episode 1 Results",
                "---",
                "Angela, Winner, +3",
                "Chuk, Loser, -1",
                "",
                "Episode 1 Standings",
                "---",
                "1. Alice: 3 (+3)",
                "2. Bob: 0",
                "",
                "Episode 2 Results",
                "---",
                "Angela Traded For Barret",
                "Angela, Winner, +3",
                "Barret, Loser, -1",
                "",
                "Episode 2 Standings",
                "---",
                "1. Bob: 3 (+3)",
                "2. Alice: 2 (-1)",
                "",
                "Episode 3 Results",
                "---",
                "Angela Traded For Chuk",
                "Barret, Winner, +3",
                "Chuk, Loser, -1",
                "",
                "Episode 3 Standings",
                "---",
                "1. Alice: 5 (+3)",
                "2. Bob: 2 (-1)",
                "",
                "",
                "Work Big Brother",
                "",
                "Houseguests",
                "---",
                "Angela",
                "Barret",
                "Chuk",
                "",
                "Draft",
                "",
                "Charles",
                "---",
                "Chuk",
                "",
                "David",
                "---",
                "Barret",
                "",
                "Episode 1 Results",
                "---",
                "Angela, Winner, +3",
                "Chuk, Loser, -1",
                "",
                "Episode 1 Standings",
                "---",
                "1. David: 0",
                "2. Charles: -1 (-1)",
                "",
                "Episode 2 Results",
                "---",
                "Angela, Winner, +3",
                "Barret, Loser, -1",
                "",
                "Episode 2 Standings",
                "---",
                "1. Charles: -1",
                "2. David: -1 (-1)",
                "",
                "Episode 3 Results",
                "---",
                "Barret, Winner, +3",
                "Chuk, Loser, -1",
                "",
                "Episode 3 Standings",
                "---",
                "1. David: 2 (+3)",
                "2. Charles: -2 (-1)",
                "",
                "",
            ]
        );
    }
}