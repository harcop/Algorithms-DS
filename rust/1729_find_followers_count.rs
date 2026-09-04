/// LeetCode #1729 - Find Followers Count (SQL; Rust analogue)
use std::collections::BTreeMap;

fn followers_count(followers: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut cnt: BTreeMap<i32, i32> = BTreeMap::new();
    for (uid, _) in followers {
        *cnt.entry(uid).or_insert(0) += 1;
    }
    cnt.into_iter().collect()
}

fn main() {
    println!("{:?}", followers_count(vec![]));
}

#[cfg(test)]
mod tests {
    use super::followers_count;

    #[test]
    fn example() {
        let followers = vec![(0, 1), (1, 0), (2, 0), (2, 1)];
        assert_eq!(followers_count(followers), vec![(0, 1), (1, 1), (2, 2)]);
    }
}
