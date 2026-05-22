/// LeetCode #1218 - Longest Arithmetic Subsequence of Given Difference
fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
    let mut dp = std::collections::HashMap::new();
    let mut best = 0i32;
    for x in arr {
        let prev = dp.get(&(x - difference)).copied().unwrap_or(0);
        let cur = prev + 1;
        dp.insert(x, cur);
        best = best.max(cur);
    }
    best
}

fn main() {
    println!("{}", longest_subsequence(vec![1, 2, 3, 4], 1));
}

#[cfg(test)]
mod tests {
    use super::longest_subsequence;

    #[test]
    fn example_one() {
        assert_eq!(longest_subsequence(vec![1, 2, 3, 4], 1), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_subsequence(vec![1, 3, 5, 7], 1), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(longest_subsequence(vec![1, 5, 7, 8, 5, 3, 4, 2, 1], -2), 4);
    }
}
