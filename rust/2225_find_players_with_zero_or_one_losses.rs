/// LeetCode #2225 - Find Players With Zero or One Losses
use std::collections::HashMap;

fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut cnt = HashMap::new();
    for m in matches {
        let winner = m[0];
        let loser = m[1];
        cnt.entry(winner).or_insert(0);
        *cnt.entry(loser).or_insert(0) += 1;
    }

    let mut ans = vec![Vec::new(), Vec::new()];
    let mut keys: Vec<i32> = cnt.keys().copied().collect();
    keys.sort_unstable();
    for x in keys {
        let v = cnt[&x];
        if v < 2 {
            ans[v as usize].push(x);
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        find_winners(vec![
            vec![1, 3],
            vec![2, 3],
            vec![3, 6],
            vec![5, 6],
            vec![5, 7],
            vec![4, 5],
            vec![4, 9],
            vec![10, 4],
            vec![4, 9],
            vec![4, 9],
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::find_winners;

    #[test]
    fn example_one() {
        assert_eq!(
            find_winners(vec![
                vec![1, 3],
                vec![2, 3],
                vec![3, 6],
                vec![5, 6],
                vec![5, 7],
                vec![4, 5],
                vec![4, 9],
                vec![10, 4],
                vec![4, 9],
                vec![4, 9],
            ]),
            vec![vec![1, 2, 10], vec![4, 5, 7]]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(find_winners(vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]]), vec![vec![1, 2, 5, 6], vec![]]);
    }
}
