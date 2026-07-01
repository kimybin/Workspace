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

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // NOTE: We use `unwrap` because we didn't deal with error handling yet.
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // TODO: Populate the scores table with the extracted details.
        // Keep in mind that goals scored by team 1 will be the number of goals
        // conceded by team 2. Similarly, goals scored by team 2 will be the
        // number of goals conceded by team 1.

        // entry(): 키가 없으면 or_insert_with()로 기본값(0,0)을 새로 넣고,
        // 있으면 기존 값에 대한 가변 참조를 돌려줌 -> 누적이 가능해짐
        let team_1 = scores.entry(team_1_name).or_insert_with(Default::default);
        team_1.goals_scored += team_1_score;
        // team_1이 넣은 골은 team_2가 먹은 골
        team_1.goals_conceded += team_2_score;

        let team_2 = scores.entry(team_2_name).or_insert_with(Default::default);
        team_2.goals_scored += team_2_score;
        // team_2가 넣은 골은 team_1이 먹은 골
        team_2.goals_conceded += team_1_score;
    }

    scores
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    // 줄을 들여쓰면 그 공백이 문자열에 그대로 포함되어 team_1_name이
    // "France" 대신 "   France"가 되는 버그가 생기므로 들여쓰기 없이 작성함
    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    // 6개 팀 이름이 모두 키로 들어있는지 체크
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    // England: 세 경기(vs France, vs Germany, vs Spain)의 득점/실점이 올바르게 누적됐는지 확인
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    // Spain: 득점 없이 실점만 누적(Poland전 2, England전 1)되는지 확인
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
