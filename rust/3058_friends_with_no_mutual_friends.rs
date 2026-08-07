/// LeetCode #3058 - Friends With No Mutual Friends (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn friends_with_no_mutual(friends: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut adj: HashMap<i32, HashSet<i32>> = HashMap::new();

    for (u, v) in &friends {
        adj.entry(*u).or_default().insert(*v);
        adj.entry(*v).or_default().insert(*u);
    }

    friends
        .into_iter()
        .filter(|(u, v)| {
            let friends_u = adj.get(u).cloned().unwrap_or_default();
            let friends_v = adj.get(v).cloned().unwrap_or_default();
            !friends_u
                .intersection(&friends_v)
                .any(|&x| x != *u && x != *v)
        })
        .collect()
}

fn main() {
    let friends = vec![
        (1, 2),
        (2, 3),
        (2, 4),
        (1, 5),
        (6, 7),
        (3, 4),
        (2, 5),
        (8, 9),
    ];
    println!("{:?}", friends_with_no_mutual(friends));
}

#[cfg(test)]
mod tests {
    use super::friends_with_no_mutual;

    #[test]
    fn example() {
        let friends = vec![
            (1, 2),
            (2, 3),
            (2, 4),
            (1, 5),
            (6, 7),
            (3, 4),
            (2, 5),
            (8, 9),
        ];
        assert_eq!(friends_with_no_mutual(friends), vec![(6, 7), (8, 9)]);
    }
}
