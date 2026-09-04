/// LeetCode #1125 - Smallest Sufficient Team
use std::collections::HashMap;

fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
    let m = req_skills.len();
    let id: HashMap<String, usize> = req_skills
        .into_iter()
        .enumerate()
        .map(|(i, s)| (s, i))
        .collect();
    let pmask: Vec<usize> = people
        .iter()
        .map(|skills| {
            let mut mask = 0usize;
            for s in skills {
                if let Some(&bit) = id.get(s) {
                    mask |= 1 << bit;
                }
            }
            mask
        })
        .collect();
    let full = (1 << m) - 1;
    let mut dp: Vec<Option<Vec<i32>>> = vec![None; 1 << m];
    dp[0] = Some(vec![]);
    for (i, &mask) in pmask.iter().enumerate() {
        let cur = dp.clone();
        for (prev, team) in cur.into_iter().enumerate() {
            let Some(team) = team else { continue };
            let nxt = prev | mask;
            let mut cand = team;
            cand.push(i as i32);
            if dp[nxt].as_ref().map_or(true, |t| cand.len() < t.len()) {
                dp[nxt] = Some(cand);
            }
        }
    }
    dp[full].clone().unwrap_or_default()
}

fn main() {
    let req = vec!["java".into(), "nodejs".into(), "reactjs".into()];
    let people = vec![
        vec!["java".into()],
        vec!["nodejs".into()],
        vec!["nodejs".into(), "reactjs".into()],
    ];
    println!("{:?}", smallest_sufficient_team(req, people));
}

#[cfg(test)]
mod tests {
    use super::smallest_sufficient_team;

    #[test]
    fn example_one() {
        let req = vec!["java".into(), "nodejs".into(), "reactjs".into()];
        let people = vec![
            vec!["java".into()],
            vec!["nodejs".into()],
            vec!["nodejs".into(), "reactjs".into()],
        ];
        let team = smallest_sufficient_team(req, people);
        assert_eq!(team, vec![0, 2]);
    }

    #[test]
    fn example_two() {
        let req = vec![
            "algorithms".into(),
            "math".into(),
            "java".into(),
            "reactjs".into(),
            "csharp".into(),
            "aws".into(),
        ];
        let people = vec![
            vec!["algorithms".into(), "math".into(), "java".into()],
            vec!["algorithms".into(), "math".into(), "reactjs".into()],
            vec!["java".into(), "csharp".into(), "aws".into()],
            vec!["reactjs".into(), "csharp".into()],
            vec!["csharp".into(), "math".into()],
            vec!["aws".into(), "java".into()],
        ];
        let team = smallest_sufficient_team(req, people);
        assert_eq!(team, vec![1, 2]);
    }
}
