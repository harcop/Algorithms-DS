/// LeetCode #397 - Integer Replacement
use std::collections::HashMap;

fn integer_replacement(n: i32) -> i32 {
    fn dfs(x: i64, memo: &mut HashMap<i64, i32>) -> i32 {
        if x == 1 {
            return 0;
        }
        if let Some(&v) = memo.get(&x) {
            return v;
        }
        let ans = if x % 2 == 0 {
            1 + dfs(x / 2, memo)
        } else {
            1 + dfs(x - 1, memo).min(dfs(x + 1, memo))
        };
        memo.insert(x, ans);
        ans
    }
    let mut memo = HashMap::new();
    dfs(n as i64, &mut memo)
}

fn main() {
    println!("{}", integer_replacement(8));
}

#[cfg(test)]
mod tests {
    use super::integer_replacement;

    #[test]
    fn example_one() {
        assert_eq!(integer_replacement(8), 3);
    }
}
