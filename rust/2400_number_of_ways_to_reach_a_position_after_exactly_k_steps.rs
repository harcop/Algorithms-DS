/// LeetCode #2400 - Number of Ways to Reach a Position After Exactly k Steps
fn number_of_ways(start_pos: i32, end_pos: i32, k: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let k = k as usize;
    let mut f = vec![vec![-1i64; k + 1]; k + 1];

    fn dfs(i: usize, j: usize, f: &mut Vec<Vec<i64>>) -> i64 {
        if i > j {
            return 0;
        }
        if j == 0 {
            return if i == 0 { 1 } else { 0 };
        }
        if f[i][j] != -1 {
            return f[i][j];
        }
        let a = dfs(i + 1, j - 1, f);
        let b = dfs(if i == 0 { 1 } else { i - 1 }, j - 1, f);
        f[i][j] = (a + b) % MOD;
        f[i][j]
    }

    let dist = (start_pos - end_pos).unsigned_abs() as usize;
    dfs(dist, k, &mut f) as i32
}

fn main() {
    println!("{}", number_of_ways(1, 2, 3));
}

#[cfg(test)]
mod tests {
    use super::number_of_ways;

    #[test]
    fn example_one() {
        assert_eq!(number_of_ways(1, 2, 3), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_ways(2, 5, 10), 0);
    }
}
