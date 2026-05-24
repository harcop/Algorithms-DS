/// LeetCode #1320 - Minimum Distance to Type a Word Using Two Fingers
fn minimum_distance(word: String) -> i32 {
    const INF: i32 = 1_000_000_000;
    let cost = |a: usize, b: usize| -> i32 {
        if a == 26 || b == 26 {
            return 0;
        }
        let (x1, y1) = ((a as i32 / 6), (a as i32 % 6));
        let (x2, y2) = ((b as i32 / 6), (b as i32 % 6));
        (x1 - x2).abs() + (y1 - y2).abs()
    };
    let mut dp = vec![vec![INF; 27]; 27];
    dp[26][26] = 0;
    for ch in word.bytes() {
        let c = (ch - b'A') as usize;
        let mut ndp = vec![vec![INF; 27]; 27];
        for f1 in 0..=26 {
            for f2 in 0..=26 {
                if dp[f1][f2] == INF {
                    continue;
                }
                ndp[c][f2] = ndp[c][f2].min(dp[f1][f2] + cost(f1, c));
                ndp[f1][c] = ndp[f1][c].min(dp[f1][f2] + cost(f2, c));
            }
        }
        dp = ndp;
    }
    dp.iter().flatten().copied().min().unwrap()
}

fn main() {
    println!("{}", minimum_distance("CAKE".to_string()));
}

#[cfg(test)]
mod tests {
    use super::minimum_distance;

    #[test]
    fn example_one() {
        assert_eq!(minimum_distance("CAKE".to_string()), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_distance("HAPPY".to_string()), 6);
    }
}
