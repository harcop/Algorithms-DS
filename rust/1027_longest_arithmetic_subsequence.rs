/// LeetCode #1027 - Longest Arithmetic Subsequence
fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let n = nums.len();
    let mut dp = vec![HashMap::new(); n];
    let mut best = 2i32;
    for i in 1..n {
        for j in 0..i {
            let diff = nums[i] - nums[j];
            let prev = *dp[j].get(&diff).unwrap_or(&1);
            let cur = prev + 1;
            dp[i].insert(diff, cur);
            best = best.max(cur);
        }
    }
    best
}

fn main() {
    println!("{}", longest_arith_seq_length(vec![3, 6, 9, 12]));
}

#[cfg(test)]
mod tests {
    use super::longest_arith_seq_length;

    #[test]
    fn example_one() {
        assert_eq!(longest_arith_seq_length(vec![3, 6, 9, 12]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_arith_seq_length(vec![9, 4, 7, 2, 10]), 3);
    }
}
