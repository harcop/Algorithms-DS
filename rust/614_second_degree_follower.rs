/// LeetCode #614 - Second Degree Follower (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn second_degree_follower(follow: Vec<(String, String)>) -> Vec<(String, i32)> {
    let mut followers: HashMap<String, HashSet<String>> = HashMap::new();
    let mut follows_someone: HashSet<String> = HashSet::new();
    for (followee, follower) in follow {
        follows_someone.insert(follower.clone());
        followers.entry(followee).or_default().insert(follower);
    }
    let mut ans: Vec<(String, i32)> = followers
        .into_iter()
        .filter(|(u, _)| follows_someone.contains(u))
        .map(|(u, fs)| (u, fs.len() as i32))
        .collect();
    ans.sort_by(|a, b| a.0.cmp(&b.0));
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::second_degree_follower;

    #[test]
    fn example() {
        let follow = vec![
            ("Alice".into(), "Bob".into()),
            ("Bob".into(), "Cena".into()),
            ("Bob".into(), "Donald".into()),
            ("Donald".into(), "Edward".into()),
        ];
        assert_eq!(
            second_degree_follower(follow),
            vec![("Bob".into(), 2), ("Donald".into(), 1)]
        );
    }
}
