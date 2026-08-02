/// LeetCode #2930 - Number of Strings Which Can Be Rearranged to Contain Substring
fn string_count(n: i32) -> i32 {
    use std::collections::HashMap;

    const MOD: i64 = 1_000_000_007;
    let mut memo = HashMap::new();

    fn dfs(
        i: i32,
        l: i32,
        e: i32,
        t: i32,
        memo: &mut HashMap<(i32, i32, i32, i32), i64>,
    ) -> i64 {
        if i == 0 {
            return if l == 1 && e == 2 && t == 1 { 1 } else { 0 };
        }
        if let Some(&v) = memo.get(&(i, l, e, t)) {
            return v;
        }
        let a = dfs(i - 1, l, e, t, memo) * 23 % MOD;
        let b = dfs(i - 1, (l + 1).min(1), e, t, memo);
        let c = dfs(i - 1, l, (e + 1).min(2), t, memo);
        let d = dfs(i - 1, l, e, (t + 1).min(1), memo);
        let ans = (a + b + c + d) % MOD;
        memo.insert((i, l, e, t), ans);
        ans
    }

    dfs(n, 0, 0, 0, &mut memo) as i32
}

fn main() {
    println!("{}", string_count(4));
}

#[cfg(test)]
mod tests {
    use super::string_count;

    #[test]
    fn example_one() {
        assert_eq!(string_count(4), 12);
    }

    #[test]
    fn example_two() {
        assert_eq!(string_count(10), 83943898);
    }
}
