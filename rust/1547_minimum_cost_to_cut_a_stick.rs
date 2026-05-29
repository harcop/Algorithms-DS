/// LeetCode #1547 - Minimum Cost To Cut A Stick
fn min_cost(n: i32, cuts: Vec<i32>) -> i32 {
    let mut a = cuts;
    a.sort_unstable();
    let mut sticks = vec![0];
    sticks.extend(a);
    sticks.push(n);
    let m = sticks.len();
    let mut dp = vec![vec![0; m]; m];
    for len in 2..m {
        for i in 0..m - len {
            let j = i + len;
            dp[i][j] = i32::MAX;
            for k in i + 1..j {
                dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j] + sticks[j] - sticks[i]);
            }
        }
    }
    dp[0][m - 1]
}

fn main() {
    println!("{}", min_cost(7, vec![1, 3, 4, 5]));
}

#[cfg(test)]
mod tests {
    use super::min_cost;

    #[test]
    fn example_one() {
        assert_eq!(min_cost(7, vec![1, 3, 4, 5]), 16);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_cost(9, vec![5, 6, 1, 4, 2]), 22);
    }
}
