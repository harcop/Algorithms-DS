/// LeetCode #1241 - Number of Comments per Post (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn number_of_comments(submissions: Vec<(i32, Option<i32>)>) -> Vec<(i32, i32)> {
    let mut posts: HashSet<i32> = HashSet::new();
    let mut comments: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (sub_id, parent) in submissions {
        match parent {
            None => {
                posts.insert(sub_id);
            }
            Some(p) => {
                comments.entry(p).or_default().insert(sub_id);
            }
        }
    }
    let mut ans: Vec<(i32, i32)> = posts
        .into_iter()
        .map(|p| (p, comments.get(&p).map(|s| s.len() as i32).unwrap_or(0)))
        .collect();
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::number_of_comments;

    #[test]
    fn example() {
        let submissions = vec![
            (1, None),
            (2, None),
            (1, None),
            (12, None),
            (3, Some(1)),
            (5, Some(2)),
            (3, Some(1)),
            (4, Some(1)),
            (9, Some(1)),
            (10, Some(2)),
            (6, Some(7)),
        ];
        assert_eq!(
            number_of_comments(submissions),
            vec![(1, 3), (2, 2), (12, 0)]
        );
    }
}
