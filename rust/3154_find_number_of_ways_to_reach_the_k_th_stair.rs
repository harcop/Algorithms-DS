/// LeetCode #3154 - Find Number of Ways to Reach the K-th Stair
use std::collections::HashMap;

fn ways_to_reach_stair(k: i32) -> i32 {
    fn dfs(
        i: i64,
        used_down: bool,
        jump: i32,
        k: i64,
        memo: &mut HashMap<(i64, bool, i32), i32>,
    ) -> i32 {
        if i > k + 1 {
            return 0;
        }
        let key = (i, used_down, jump);
        if let Some(&v) = memo.get(&key) {
            return v;
        }
        let mut ans = if i == k { 1 } else { 0 };
        if i > 0 && !used_down {
            ans += dfs(i - 1, true, jump, k, memo);
        }
        ans += dfs(i + (1i64 << jump), false, jump + 1, k, memo);
        memo.insert(key, ans);
        ans
    }
    let mut memo = HashMap::new();
    dfs(1, false, 0, k as i64, &mut memo)
}

fn main() {
    println!("{}", ways_to_reach_stair(0));
}

#[cfg(test)]
mod tests {
    use super::ways_to_reach_stair;

    #[test]
    fn example1() {
        assert_eq!(ways_to_reach_stair(0), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(ways_to_reach_stair(1), 4);
    }
}
