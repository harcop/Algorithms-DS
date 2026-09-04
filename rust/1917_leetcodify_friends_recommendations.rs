/// LeetCode #1917 - Leetcodify Friends Recommendations (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn friends_recommendations(
    listens: Vec<(i32, i32, String)>,
    friendship: Vec<(i32, i32)>,
) -> Vec<(i32, i32)> {
    let mut friends: HashSet<(i32, i32)> = HashSet::new();
    for (a, b) in friendship {
        friends.insert((a, b));
        friends.insert((b, a));
    }
    let mut by_day: HashMap<String, HashMap<i32, HashSet<i32>>> = HashMap::new();
    for (user_id, song_id, day) in listens {
        by_day
            .entry(day)
            .or_default()
            .entry(user_id)
            .or_default()
            .insert(song_id);
    }
    let mut rec: HashSet<(i32, i32)> = HashSet::new();
    for users in by_day.values() {
        let ids: Vec<i32> = users.keys().copied().collect();
        for i in 0..ids.len() {
            for j in 0..ids.len() {
                if i == j {
                    continue;
                }
                let a = ids[i];
                let b = ids[j];
                if friends.contains(&(a, b)) {
                    continue;
                }
                let common = users[&a].intersection(&users[&b]).count();
                if common >= 3 {
                    rec.insert((a, b));
                }
            }
        }
    }
    let mut ans: Vec<(i32, i32)> = rec.into_iter().collect();
    ans.sort();
    ans
}

fn main() {
    println!("{:?}", friends_recommendations(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::friends_recommendations;

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
        let friendship = vec![(1, 2)];
        assert_eq!(
            friends_recommendations(listens, friendship),
            vec![(1, 3), (2, 3), (3, 1), (3, 2)]
        );
    }
}
