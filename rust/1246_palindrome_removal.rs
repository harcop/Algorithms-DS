/// LeetCode #1246 - Palindrome Removal
fn minimum_moves(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut dp = vec![vec![0i32; n]; n];
    for len in 1..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            if i == j {
                dp[i][j] = 1;
            } else if arr[i] == arr[j] && (len == 2 || dp[i + 1][j - 1] == 1) {
                dp[i][j] = 1;
            } else {
                let mut best = dp[i + 1][j] + dp[i][i];
                for k in i..j {
                    best = best.min(dp[i][k] + dp[k + 1][j]);
                }
                dp[i][j] = best;
            }
        }
    }
    dp[0][n - 1]
}

fn main() {
    println!("{}", minimum_moves(vec![1, 3, 4, 1]));
}

#[cfg(test)]
mod tests {
    use super::minimum_moves;

    #[test]
    fn example_one() {
        assert_eq!(minimum_moves(vec![1, 3, 4, 1]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_moves(vec![3, 2, 1, 4]), 4);
    }
}
