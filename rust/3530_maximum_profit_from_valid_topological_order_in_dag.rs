/// LeetCode #3530 - Maximum Profit from Valid Topological Order in DAG
fn max_profit(n: i32, edges: Vec<Vec<i32>>, score: Vec<i32>) -> i32 {
    let n = n as usize;
    let mut need = vec![0usize; n];
    for e in &edges {
        need[e[1] as usize] |= 1 << e[0];
    }
    let max_mask = 1 << n;
    let mut dp = vec![-1i32; max_mask];
    dp[0] = 0;
    for mask in 0..max_mask {
        if dp[mask] < 0 {
            continue;
        }
        let pos = (mask.count_ones() as i32) + 1;
        for i in 0..n {
            if (mask >> i) & 1 == 1 {
                continue;
            }
            if (mask & need[i]) == need[i] {
                let new_mask = mask | (1 << i);
                dp[new_mask] = dp[new_mask].max(dp[mask] + score[i] * pos);
            }
        }
    }
    dp[max_mask - 1]
}

fn main() {
    println!("{}", max_profit(2, vec![vec![0, 1]], vec![2, 3]));
}

#[cfg(test)]
mod tests {
    use super::max_profit;

    #[test]
    fn example1() {
        assert_eq!(max_profit(2, vec![vec![0, 1]], vec![2, 3]), 8);
    }

    #[test]
    fn example2() {
        assert_eq!(max_profit(3, vec![vec![0, 1], vec![0, 2]], vec![1, 6, 3]), 25);
    }
}
