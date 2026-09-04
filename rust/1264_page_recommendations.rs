/// LeetCode #1264 - Page Recommendations (SQL; Rust analogue)
use std::collections::HashSet;

fn page_recommendations(
    friendship: Vec<(i32, i32)>,
    likes: Vec<(i32, i32)>,
) -> Vec<i32> {
    let mut friends = HashSet::new();
    for (a, b) in friendship {
        if a == 1 {
            friends.insert(b);
        } else if b == 1 {
            friends.insert(a);
        }
    }
    let mut mine = HashSet::new();
    let mut rec = HashSet::new();
    for (uid, page) in likes {
        if uid == 1 {
            mine.insert(page);
        } else if friends.contains(&uid) {
            rec.insert(page);
        }
    }
    let mut ans: Vec<i32> = rec.difference(&mine).copied().collect();
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::page_recommendations;

    #[test]
    fn example() {
        let friendship = vec![(1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (2, 5), (6, 1)];
        let likes = vec![
            (1, 88),
            (2, 23),
            (3, 24),
            (4, 56),
            (5, 11),
            (6, 33),
            (2, 77),
            (3, 77),
            (6, 88),
        ];
        assert_eq!(page_recommendations(friendship, likes), vec![23, 24, 33, 56, 77]);
    }
}
