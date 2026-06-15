/// LeetCode #1872 - Stone Game VIII
fn stone_game_viii(stones: Vec<i32>) -> i32 {
    let n = stones.len();
    let mut s = vec![0i32; n];
    s[0] = stones[0];
    for i in 1..n {
        s[i] = s[i - 1] + stones[i];
    }
    let mut memo = vec![None; n];
    fn dfs(i: usize, s: &[i32], memo: &mut [Option<i32>]) -> i32 {
        if i >= s.len() - 1 {
            return s[s.len() - 1];
        }
        if let Some(v) = memo[i] {
            return v;
        }
        let v = dfs(i + 1, s, memo).max(s[i] - dfs(i + 1, s, memo));
        memo[i] = Some(v);
        v
    }
    dfs(1, &s, &mut memo)
}

fn main() {
    println!("{}", stone_game_viii(vec![-1, 2, -3, 4, -5]));
}

#[cfg(test)]
mod tests {
    use super::stone_game_viii;

    #[test]
    fn example_one() {
        assert_eq!(stone_game_viii(vec![-1, 2, -3, 4, -5]), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(stone_game_viii(vec![7, -6, 5, 10, 5, -2, -6]), 13);
    }
}
