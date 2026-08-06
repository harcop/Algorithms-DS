/// LeetCode #3051 - Find Candidates for Data Scientist Position (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn find_candidates(candidates: Vec<(i32, String)>) -> Vec<i32> {
    let required: HashSet<&str> = ["Python", "Tableau", "PostgreSQL"].into_iter().collect();
    let mut by_id: HashMap<i32, HashSet<String>> = HashMap::new();

    for (id, skill) in candidates {
        by_id.entry(id).or_default().insert(skill);
    }

    let mut ans: Vec<_> = by_id
        .into_iter()
        .filter(|(_, skills)| required.iter().all(|s| skills.contains(*s)))
        .map(|(id, _)| id)
        .collect();
    ans.sort_unstable();
    ans
}

fn main() {
    let candidates = vec![
        (123, "Python".into()),
        (234, "R Programming".into()),
        (123, "Tableau".into()),
        (123, "PostgreSQL".into()),
        (234, "PowerBI".into()),
        (234, "SQL Server".into()),
        (147, "Python".into()),
        (147, "PostgreSQL".into()),
        (147, "Tableau".into()),
        (147, "PowerBI".into()),
        (192, "PostgreSQL".into()),
        (192, "Tableau".into()),
        (192, "Java".into()),
    ];
    println!("{:?}", find_candidates(candidates));
}

#[cfg(test)]
mod tests {
    use super::find_candidates;

    #[test]
    fn example() {
        let candidates = vec![
            (123, "Python".into()),
            (234, "R Programming".into()),
            (123, "Tableau".into()),
            (123, "PostgreSQL".into()),
            (234, "PowerBI".into()),
            (234, "SQL Server".into()),
            (147, "Python".into()),
            (147, "PostgreSQL".into()),
            (147, "Tableau".into()),
            (147, "PowerBI".into()),
            (192, "PostgreSQL".into()),
            (192, "Tableau".into()),
            (192, "Java".into()),
        ];
        assert_eq!(find_candidates(candidates), vec![123, 147]);
    }
}
