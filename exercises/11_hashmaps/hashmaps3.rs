// A list of scores (one per line) of a soccer match is given. Each line is of
// the form "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: "England,France,4,2" (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, the total
// number of goals the team scored, and the total number of goals the team
// conceded.

use std::collections::HashMap;

// A structure to store the goal details of a team.
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores = HashMap::<&str, TeamScores>::new();
    results
        .lines()
        .map(|line| line.split(','))
        .for_each(|mut splitted_line| {
            let team1 = splitted_line.next().unwrap();
            let team2 = splitted_line.next().unwrap();
            let score1 = splitted_line.next().unwrap().parse::<u8>().unwrap();
            let score2 = splitted_line.next().unwrap().parse::<u8>().unwrap();
            scores.entry(team1).and_modify(|score| {
                score.goals_scored += score1;
                score.goals_conceded += score2;
            }).or_insert(TeamScores { goals_scored: score1, goals_conceded: score2 });
            scores.entry(team2).and_modify(|score| {
                score.goals_scored += score2;
                score.goals_conceded += score1;
            }).or_insert(TeamScores { goals_scored: score2, goals_conceded: score1 });
        });
    scores
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
