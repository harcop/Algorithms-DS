/// LeetCode #1366 - Rank Teams By Votes

fn rank_teams(votes: Vec<String>) -> String {
    if votes.is_empty() {
        return String::new();
    }
    let k = votes[0].len();
    let mut teams: Vec<char> = votes[0].chars().collect();
    teams.sort_unstable();
    teams.dedup();
    let mut scores: std::collections::HashMap<char, Vec<i32>> =
        teams.iter().map(|&c| (c, vec![0; k])).collect();
    for vote in &votes {
        for (pos, ch) in vote.chars().enumerate() {
            scores.get_mut(&ch).unwrap()[pos] += 1;
        }
    }
    teams.sort_by(|&a, &b| {
        let sa = &scores[&a];
        let sb = &scores[&b];
        for i in 0..k {
            match sb[i].cmp(&sa[i]) {
                std::cmp::Ordering::Equal => continue,
                other => return other,
            }
        }
        a.cmp(&b)
    });
    teams.into_iter().collect()
}

fn main() {
    println!("{}", rank_teams(vec!["ABC".into(), "ACB".into(), "ABC".into(), "ACB".into(), "ACB".into()]));
}

#[cfg(test)]
mod tests {
    use super::rank_teams;

    #[test]
    fn example_one() {
        assert_eq!(
            rank_teams(vec!["ABC".into(), "ACB".into(), "ABC".into(), "ACB".into(), "ACB".into()]),
            "ACB"
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(rank_teams(vec!["WXYZ".into(), "XYWZ".into()]), "XWYZ");
    }
}
