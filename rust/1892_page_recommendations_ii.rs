/// LeetCode #1892 - Page Recommendations II (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn page_recommendations_ii(
    friendship: Vec<(i32, i32)>,
    likes: Vec<(i32, i32)>,
) -> Vec<(i32, i32, i32)> {
    let mut friends: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (a, b) in friendship {
        friends.entry(a).or_default().insert(b);
        friends.entry(b).or_default().insert(a);
    }
    let mut liked: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (user_id, page_id) in likes {
        liked.entry(user_id).or_default().insert(page_id);
    }
    let mut rec: HashMap<(i32, i32), i32> = HashMap::new();
    for (user_id, frs) in &friends {
        let mine = liked.get(user_id).cloned().unwrap_or_default();
        for f in frs {
            if let Some(pages) = liked.get(f) {
                for &page in pages {
                    if !mine.contains(&page) {
                        *rec.entry((*user_id, page)).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    let mut ans: Vec<(i32, i32, i32)> = rec
        .into_iter()
        .map(|((u, p), c)| (u, p, c))
        .collect();
    ans.sort();
    ans
}

fn main() {
    println!("{:?}", page_recommendations_ii(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::page_recommendations_ii;

    #[test]
    fn example_one() {
        let friendship = vec![
            (1, 2),
            (1, 3),
            (1, 4),
            (2, 3),
            (2, 4),
            (2, 5),
            (6, 1),
        ];
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
        let mut expected = vec![
            (1, 77, 2),
            (1, 23, 1),
            (1, 24, 1),
            (1, 56, 1),
            (1, 33, 1),
            (2, 24, 1),
            (2, 56, 1),
            (2, 11, 1),
            (2, 88, 1),
            (3, 88, 1),
            (3, 23, 1),
            (4, 88, 1),
            (4, 77, 1),
            (4, 23, 1),
            (5, 77, 1),
            (5, 23, 1),
        ];
        expected.sort();
        assert_eq!(page_recommendations_ii(friendship, likes), expected);
    }
}
