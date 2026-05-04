/// LeetCode #279 - Perfect Squares
fn num_squares(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![0; n + 1];
    for i in 1..=n {
        let mut best = i;
        let mut j = 1usize;
        while j * j <= i {
            best = best.min(dp[i - j * j]);
            j += 1;
        }
        dp[i] = best + 1;
    }
    dp[n] as i32
}

fn main() {
    println!("{}", num_squares(12));
}

#[cfg(test)]
mod tests {
    use super::num_squares;

    #[test]
    fn example_one() {
        assert_eq!(num_squares(12), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_squares(13), 2);
    }
}
