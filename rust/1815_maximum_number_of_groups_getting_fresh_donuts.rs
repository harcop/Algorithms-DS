/// LeetCode #1815 - Maximum Number of Groups Getting Fresh Donuts
use std::collections::HashMap;

fn max_happy_groups(batch_size: i32, groups: Vec<i32>) -> i32 {
    let b = batch_size as usize;
    let mut state = 0u64;
    let mut ans = 0i32;
    for v in groups {
        let i = (v % batch_size) as usize;
        if i == 0 {
            ans += 1;
        } else {
            state += 1u64 << (i * 5);
        }
    }

    fn dfs(
        state: u64,
        rem: usize,
        b: usize,
        memo: &mut HashMap<(u64, usize), i32>,
    ) -> i32 {
        if let Some(&v) = memo.get(&(state, rem)) {
            return v;
        }
        let mut res = 0i32;
        let bonus = if rem == 0 { 1 } else { 0 };
        for i in 1..b {
            if (state >> (i * 5)) & 31 > 0 {
                let next_state = state - (1u64 << (i * 5));
                let next_rem = (rem + i) % b;
                res = res.max(dfs(next_state, next_rem, b, memo) + bonus);
            }
        }
        memo.insert((state, rem), res);
        res
    }

    let mut memo = HashMap::new();
    ans + dfs(state, 0, b, &mut memo)
}

fn main() {
    println!("{}", max_happy_groups(3, vec![1, 2, 3, 4, 5, 6]));
}

#[cfg(test)]
mod tests {
    use super::max_happy_groups;

    #[test]
    fn example_one() {
        assert_eq!(max_happy_groups(3, vec![1, 2, 3, 4, 5, 6]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_happy_groups(4, vec![1, 3, 2, 5, 2, 2, 1, 6]), 4);
    }
}
