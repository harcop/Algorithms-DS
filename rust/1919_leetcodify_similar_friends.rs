/// LeetCode #1919 - Leetcodify Similar Friends (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn similar_friends(
    listens: Vec<(i32, i32, String)>,
    friendship: Vec<(i32, i32)>,
) -> Vec<(i32, i32)> {
    let mut by_day: HashMap<String, HashMap<i32, HashSet<i32>>> = HashMap::new();
    for (user_id, song_id, day) in listens {
        by_day
            .entry(day)
            .or_default()
            .entry(user_id)
            .or_default()
            .insert(song_id);
    }
    let mut ans = Vec::new();
    for (a, b) in friendship {
        let similar = by_day.values().any(|users| {
            match (users.get(&a), users.get(&b)) {
                (Some(sa), Some(sb)) => sa.intersection(sb).count() >= 3,
                _ => false,
            }
        });
        if similar {
            ans.push((a, b));
        }
    }
    ans.sort();
    ans
}

fn main() {
    println!("{:?}", similar_friends(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::similar_friends;

    #[test]
    fn example_one() {
        let listens = vec![
            (1, 10, "2021-03-15".into()),
            (1, 11, "2021-03-15".into()),
            (1, 12, "2021-03-15".into()),
            (2, 10, "2021-03-15".into()),
            (2, 11, "2021-03-15".into()),
            (2, 12, "2021-03-15".into()),
            (3, 10, "2021-03-15".into()),
            (3, 11, "2021-03-15".into()),
            (3, 12, "2021-03-15".into()),
            (4, 10, "2021-03-15".into()),
            (4, 11, "2021-03-15".into()),
            (4, 13, "2021-03-15".into()),
            (5, 10, "2021-03-16".into()),
            (5, 11, "2021-03-16".into()),
            (5, 12, "2021-03-16".into()),
        ];
        let friendship = vec![(1, 2), (2, 4), (2, 5)];
        assert_eq!(similar_friends(listens, friendship), vec![(1, 2)]);
    }
}
