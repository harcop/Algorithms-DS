/// LeetCode #1949 - Strong Friendship (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn strong_friendship(friendship: Vec<(i32, i32)>) -> Vec<(i32, i32, i32)> {
    let mut friends: HashMap<i32, HashSet<i32>> = HashMap::new();
    for &(a, b) in &friendship {
        friends.entry(a).or_default().insert(b);
        friends.entry(b).or_default().insert(a);
    }
    let mut ans = Vec::new();
    for (a, b) in friendship {
        let fa = friends.get(&a).unwrap();
        let fb = friends.get(&b).unwrap();
        let common = fa.intersection(fb).count() as i32;
        if common >= 3 {
            ans.push((a.min(b), a.max(b), common));
        }
    }
    ans.sort();
    ans
}

fn main() {
    println!("{:?}", strong_friendship(vec![]));
}

#[cfg(test)]
mod tests {
    use super::strong_friendship;

    #[test]
    fn example_one() {
        let friendship = vec![
            (1, 2),
            (1, 3),
            (2, 3),
            (1, 4),
            (2, 4),
            (1, 5),
            (2, 5),
            (1, 7),
            (3, 7),
            (1, 6),
            (3, 6),
            (2, 6),
        ];
        assert_eq!(
            strong_friendship(friendship),
            vec![(1, 2, 4), (1, 3, 3)]
        );
    }
}
