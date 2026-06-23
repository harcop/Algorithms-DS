/// LeetCode #2052 - Minimum Cost to Separate Sentence Into Rows
use std::collections::HashMap;

fn minimum_cost(sentence: String, k: i32) -> i32 {
    let k = k as i32;
    let lens: Vec<i32> = sentence
        .split_whitespace()
        .map(|w| w.len() as i32)
        .collect();
    let n = lens.len();
    let mut prefix = vec![0i32; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + lens[i];
    }

    let mut memo = HashMap::new();
    fn dfs(
        i: usize,
        n: usize,
        k: i32,
        prefix: &[i32],
        memo: &mut HashMap<usize, i32>,
    ) -> i32 {
        if let Some(&v) = memo.get(&i) {
            return v;
        }
        let remaining = prefix[n] - prefix[i] + (n - i - 1) as i32;
        if remaining <= k {
            memo.insert(i, 0);
            return 0;
        }
        let mut ans = i32::MAX / 2;
        let mut j = i + 1;
        while j < n {
            let m = prefix[j] - prefix[i] + (j - i - 1) as i32;
            if m > k {
                break;
            }
            ans = ans.min(dfs(j, n, k, prefix, memo) + (k - m).pow(2));
            j += 1;
        }
        memo.insert(i, ans);
        ans
    }

    dfs(0, n, k, &prefix, &mut memo)
}

fn main() {
    println!(
        "{}",
        minimum_cost("i love leetcode".into(), 12)
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_cost;

    #[test]
    fn example_one() {
        assert_eq!(minimum_cost("i love leetcode".into(), 12), 36);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            minimum_cost("apples and bananas taste great".into(), 7),
            21
        );
    }
}
