/// LeetCode #1951 - All the Pairs With the Maximum Number of Common Followers (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn max_common_followers(relations: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut followers: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (user_id, follower_id) in relations {
        followers.entry(user_id).or_default().insert(follower_id);
    }
    let users: Vec<i32> = followers.keys().copied().collect();
    let mut pairs: Vec<(i32, i32, usize)> = Vec::new();
    let mut max_c = 0;
    for i in 0..users.len() {
        for j in i + 1..users.len() {
            let a = users[i].min(users[j]);
            let b = users[i].max(users[j]);
            let c = followers[&a].intersection(&followers[&b]).count();
            max_c = max_c.max(c);
            pairs.push((a, b, c));
        }
    }
    let mut ans: Vec<(i32, i32)> = pairs
        .into_iter()
        .filter(|(_, _, c)| *c == max_c && max_c > 0)
        .map(|(a, b, _)| (a, b))
        .collect();
    ans.sort();
    ans
}

fn main() {
    println!("{:?}", max_common_followers(vec![]));
}

#[cfg(test)]
mod tests {
    use super::max_common_followers;

    #[test]
    fn example_one() {
        let relations = vec![
            (1, 3),
            (2, 3),
            (7, 3),
            (1, 4),
            (2, 4),
            (7, 4),
            (1, 5),
            (2, 6),
            (7, 5),
        ];
        assert_eq!(max_common_followers(relations), vec![(1, 7)]);
    }
}
