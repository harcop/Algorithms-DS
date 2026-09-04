/// LeetCode #1050 - Actors and Directors Who Cooperated At Least Three Times (SQL; Rust analogue)
use std::collections::HashMap;

fn actors_directors(actor_director: Vec<(i32, i32, i32)>) -> Vec<(i32, i32)> {
    let mut cnt: HashMap<(i32, i32), i32> = HashMap::new();
    for (a, d, _) in actor_director {
        *cnt.entry((a, d)).or_insert(0) += 1;
    }
    let mut ans: Vec<(i32, i32)> = cnt
        .into_iter()
        .filter(|(_, c)| *c >= 3)
        .map(|(k, _)| k)
        .collect();
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::actors_directors;

    #[test]
    fn example() {
        let rows = vec![
            (1, 1, 0),
            (1, 1, 1),
            (1, 1, 2),
            (1, 2, 3),
            (1, 2, 4),
            (2, 1, 5),
            (2, 1, 6),
        ];
        assert_eq!(actors_directors(rows), vec![(1, 1)]);
    }
}
