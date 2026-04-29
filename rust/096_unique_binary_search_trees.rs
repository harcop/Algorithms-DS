/// LeetCode #96 - Unique Binary Search Trees
fn num_trees(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![0i32; n + 1];
    dp[0] = 1;
    dp[1] = 1;
    for i in 2..=n {
        for j in 1..=i {
            dp[i] += dp[j - 1] * dp[i - j];
        }
    }
    dp[n]
}

fn main() {
    println!("{}", num_trees(3));
}

#[cfg(test)]
mod tests {
    use super::num_trees;

    #[test]
    fn example_one() {
        assert_eq!(num_trees(3), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_trees(1), 1);
    }
}
