/// LeetCode #576 - Out of Boundary Paths
use std::collections::HashMap;

fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut memo = HashMap::new();
    fn dfs(
        i: i32,
        j: i32,
        k: i32,
        m: i32,
        n: i32,
        memo: &mut HashMap<(i32, i32, i32), i64>,
    ) -> i64 {
        if i < 0 || i >= m || j < 0 || j >= n {
            return 1;
        }
        if k == 0 {
            return 0;
        }
        let key = (i, j, k);
        if let Some(&v) = memo.get(&key) {
            return v;
        }
        let mut ans = 0i64;
        for (di, dj) in [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)] {
            ans = (ans + dfs(i + di, j + dj, k - 1, m, n, memo)) % MOD;
        }
        memo.insert(key, ans);
        ans
    }
    dfs(start_row, start_column, max_move, m, n, &mut memo) as i32
}

fn main() {
    println!("{}", find_paths(2, 2, 2, 0, 0));
}

#[cfg(test)]
mod tests {
    use super::find_paths;

    #[test]
    fn example_one() {
        assert_eq!(find_paths(2, 2, 2, 0, 0), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_paths(1, 3, 3, 0, 1), 12);
    }
}
