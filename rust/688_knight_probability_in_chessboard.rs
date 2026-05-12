/// LeetCode #688 - Knight Probability in Chessboard
use std::collections::HashMap;

fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
    let mut memo: HashMap<(i32, i32, i32), f64> = HashMap::new();
    fn dfs(r: i32, c: i32, k: i32, n: i32, memo: &mut HashMap<(i32, i32, i32), f64>) -> f64 {
        if r < 0 || r >= n || c < 0 || c >= n {
            return 0.0;
        }
        if k == 0 {
            return 1.0;
        }
        let key = (r, c, k);
        if let Some(&v) = memo.get(&key) {
            return v;
        }
        let moves = [
            (-2, -1),
            (-2, 1),
            (-1, -2),
            (-1, 2),
            (1, -2),
            (1, 2),
            (2, -1),
            (2, 1),
        ];
        let mut sum = 0.0f64;
        for (dr, dc) in moves {
            sum += dfs(r + dr, c + dc, k - 1, n, memo) / 8.0;
        }
        memo.insert(key, sum);
        sum
    }
    dfs(row, column, k, n, &mut memo)
}

fn main() {
    println!("{}", knight_probability(3, 2, 0, 0));
}

#[cfg(test)]
mod tests {
    use super::knight_probability;

    #[test]
    fn example_one() {
        let p = knight_probability(3, 2, 0, 0);
        assert!((p - 0.0625).abs() < 1e-6);
    }

    #[test]
    fn example_two() {
        let p = knight_probability(1, 0, 0, 0);
        assert!((p - 1.0).abs() < 1e-6);
    }
}
