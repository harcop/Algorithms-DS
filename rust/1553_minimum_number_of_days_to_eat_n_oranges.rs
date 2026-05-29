/// LeetCode #1553 - Minimum Number Of Days To Eat N Oranges
use std::collections::HashMap;

fn min_days(n: i32) -> i32 {
    fn dfs(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if n <= 1 {
            return n;
        }
        if let Some(&v) = memo.get(&n) {
            return v;
        }
        let ans = 1 + (n % 2 + dfs(n / 2, memo)).min(n % 3 + dfs(n / 3, memo));
        memo.insert(n, ans);
        ans
    }
    let mut memo = HashMap::new();
    dfs(n, &mut memo)
}

fn main() {
    println!("{}", min_days(10));
}

#[cfg(test)]
mod tests {
    use super::min_days;

    #[test]
    fn example_one() {
        assert_eq!(min_days(10), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_days(6), 3);
    }
}
