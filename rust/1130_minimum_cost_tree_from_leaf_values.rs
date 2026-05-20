/// LeetCode #1130 - Minimum Cost Tree From Leaf Values
fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    if n == 1 {
        return 0;
    }
    let mut dp = vec![vec![0i32; n]; n];
    let mut mx = vec![vec![0i32; n]; n];
    for i in 0..n {
        mx[i][i] = arr[i];
    }
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            dp[i][j] = i32::MAX;
            for k in i..j {
                let cost = dp[i][k] + dp[k + 1][j] + mx[i][k] * mx[k + 1][j];
                dp[i][j] = dp[i][j].min(cost);
                mx[i][j] = mx[i][k].max(mx[k + 1][j]);
            }
        }
    }
    dp[0][n - 1]
}

fn main() {
    println!("{}", mct_from_leaf_values(vec![6, 2, 4]));
}

#[cfg(test)]
mod tests {
    use super::mct_from_leaf_values;

    #[test]
    fn example_one() {
        assert_eq!(mct_from_leaf_values(vec![6, 2, 4]), 32);
    }

    #[test]
    fn example_two() {
        assert_eq!(mct_from_leaf_values(vec![4, 11]), 44);
    }
}
