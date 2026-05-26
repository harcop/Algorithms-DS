/// LeetCode #1388 - Pizza With 3n Slices
fn max_size_slices(slices: Vec<i32>) -> i32 {
    let k = slices.len() / 3;
    fn pick(arr: &[i32], t: usize) -> i32 {
        let n = arr.len();
        let mut dp = vec![vec![i32::MIN / 4; t + 1]; n + 1];
        dp[0][0] = 0;
        for i in 1..=n {
            for j in 0..=t {
                dp[i][j] = dp[i - 1][j];
                if j > 0 {
                    let prev = if i >= 2 { dp[i - 2][j - 1] } else { 0 };
                    dp[i][j] = dp[i][j].max(prev + arr[i - 1]);
                }
            }
        }
        dp[n][t]
    }
    let n = slices.len();
    pick(&slices[1..], k).max(pick(&slices[..n - 1], k))
}

fn main() {
    println!("{}", max_size_slices(vec![1, 2, 3, 4, 5, 6]));
}

#[cfg(test)]
mod tests {
    use super::max_size_slices;

    #[test]
    fn example_one() {
        assert_eq!(max_size_slices(vec![1, 2, 3, 4, 5, 6]), 10);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_size_slices(vec![8, 9, 8, 6, 1, 1]), 16);
    }
}

