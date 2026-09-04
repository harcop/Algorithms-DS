/// LeetCode #1355 - Activity Participants (SQL; Rust analogue)
use std::collections::HashMap;

fn activity_participants(
    friends: Vec<(i32, String, String)>,
    _activities: Vec<(i32, String)>,
) -> Vec<String> {
    let mut cnt: HashMap<String, i32> = HashMap::new();
    for (_, _, act) in friends {
        *cnt.entry(act).or_insert(0) += 1;
    }
    let min = *cnt.values().min().unwrap_or(&0);
    let max = *cnt.values().max().unwrap_or(&0);
    cnt.into_iter()
        .filter(|(_, c)| *c > min && *c < max)
        .map(|(a, _)| a)
        .collect()
}

fn main() {
    println!("{:?}", activity_participants(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::activity_participants;

    #[test]
    fn example() {
        let friends = vec![
            (1, "Jonathan D.".into(), "Eating".into()),
            (2, "Jade W.".into(), "Singing".into()),
            (3, "Victor J.".into(), "Singing".into()),
            (4, "Elvis Q.".into(), "Eating".into()),
            (5, "Daniel A.".into(), "Eating".into()),
            (6, "Bob B.".into(), "Horse Riding".into()),
        ];
        let activities = vec![
            (1, "Eating".into()),
            (2, "Singing".into()),
            (3, "Horse Riding".into()),
        ];
        assert_eq!(
            activity_participants(friends, activities),
            vec!["Singing".to_string()]
        );
    }
}
