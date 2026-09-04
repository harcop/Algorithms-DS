/// LeetCode #465 - Optimal Account Balancing
use std::collections::HashMap;

fn min_transfers(transactions: Vec<Vec<i32>>) -> i32 {
    let mut bal = HashMap::new();
    for t in transactions {
        *bal.entry(t[0]).or_insert(0) += t[2];
        *bal.entry(t[1]).or_insert(0) -= t[2];
    }
    let mut debts: Vec<i32> = bal.into_values().filter(|&x| x != 0).collect();
    dfs(&mut debts, 0)
}

fn dfs(debts: &mut [i32], start: usize) -> i32 {
    let mut i = start;
    while i < debts.len() && debts[i] == 0 {
        i += 1;
    }
    if i == debts.len() {
        return 0;
    }
    let mut ans = i32::MAX;
    for j in i + 1..debts.len() {
        if debts[j] * debts[i] < 0 {
            debts[j] += debts[i];
            ans = ans.min(1 + dfs(debts, i + 1));
            debts[j] -= debts[i];
        }
    }
    if ans == i32::MAX {
        0
    } else {
        ans
    }
}

fn main() {
    println!(
        "{}",
        min_transfers(vec![vec![0, 1, 10], vec![1, 0, 1], vec![1, 2, 5], vec![2, 0, 5]])
    );
}

#[cfg(test)]
mod tests {
    use super::min_transfers;

    #[test]
    fn example_one() {
        assert_eq!(
            min_transfers(vec![vec![0, 1, 10], vec![2, 0, 5]]),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            min_transfers(vec![
                vec![0, 1, 10],
                vec![1, 0, 1],
                vec![1, 2, 5],
                vec![2, 0, 5]
            ]),
            1
        );
    }
}
