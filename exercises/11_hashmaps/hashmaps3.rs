// A list of scores (one per line) of a soccer match is given. Each line is of
// the form "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: "England,France,4,2" (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, the total
// number of goals the team scored, and the total number of goals the team
// conceded.
// 축구 경기 점수 목록이 한 줄에 하나씩 주어집니다. 각 줄은
// "<팀_1_이름>,<팀_2_이름>,<팀_1_득점>,<팀_2_득점>" 형식입니다.
// 예: "England,France,4,2" (영국 4점, 프랑스 2점).
//
// 팀 이름, 팀의 총 득점 수, 팀의 총 실점 수를 포함하는
// 점수표를 만들어야 합니다.

use std::collections::HashMap;

// A structure to store the goal details of a team.
// 팀의 득점 세부 정보를 저장하는 구조체입니다.
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
    // The name of the team is the key and its associated struct is the value.
    // 팀 이름이 키(key)이고 관련 구조체가 값(value)입니다.
    let mut scores = HashMap::<&str, TeamScores>::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // NOTE: We use `unwrap` because we didn't deal with error handling yet.
        // 참고: 아직 에러 처리를 다루지 않았기 때문에 `unwrap`을 사용합니다.
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // TODO: Populate the scores table with the extracted details.
        // Keep in mind that goals scored by team 1 will be the number of goals
        // conceded by team 2. Similarly, goals scored by team 2 will be the
        // number of goals conceded by team 1.
        // TODO: 추출된 세부 정보로 점수표를 채우세요.
        // 팀 1이 득점한 골은 팀 2가 실점한 골이 된다는 점을 명심하세요.
        // 마찬가지로 팀 2가 득점한 골은 팀 1이 실점한 골이 됩니다.

        let team_1 = scores.entry(team_1_name).or_default();
        team_1.goals_scored += team_1_score;
        team_1.goals_conceded += team_2_score;

        let team_2 = scores.entry(team_2_name).or_default();
        team_2.goals_scored += team_2_score;
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
