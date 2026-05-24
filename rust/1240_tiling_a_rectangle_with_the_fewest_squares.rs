/// LeetCode #1240 - Tiling a Rectangle with the Fewest Squares
use std::collections::HashMap;

fn tiling_rectangle(n: i32, m: i32) -> i32 {
    fn dfs(n: i32, m: i32, memo: &mut HashMap<(i32, i32), i32>) -> i32 {
        if n == 0 || m == 0 {
            return 0;
        }
        let (n, m) = if n <= m { (n, m) } else { (m, n) };
        if n == m {
            return 1;
        }
        if let Some(&v) = memo.get(&(n, m)) {
            return v;
        }
        let mut ans = i32::MAX;
        for i in 1..n {
            ans = ans.min(dfs(n - i, m, memo) + dfs(i, m - i, memo) + 1);
        }
        for i in 1..=n {
            ans = ans.min(dfs(n, m - i, memo) + dfs(i, n - i + m - i, memo) + 1);
        }
        memo.insert((n, m), ans);
        ans
    }
    dfs(n, m, &mut HashMap::new())
}

fn main() {
    println!("{}", tiling_rectangle(2, 3));
}

#[cfg(test)]
mod tests {
    use super::tiling_rectangle;

    #[test]
    fn example_one() {
        assert_eq!(tiling_rectangle(2, 3), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(tiling_rectangle(5, 8), 5);
    }

    #[test]
    fn example_three() {
        assert_eq!(tiling_rectangle(11, 13), 6);
    }
}
