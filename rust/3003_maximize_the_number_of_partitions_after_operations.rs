/// LeetCode #3003 - Maximize the Number of Partitions After Operations
use std::collections::HashMap;

fn max_partitions_after_operations(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let k = k as u32;

    fn dfs(
        i: usize,
        cur: u32,
        t: bool,
        s: &[u8],
        n: usize,
        k: u32,
        memo: &mut HashMap<(usize, u32, u8), i32>,
    ) -> i32 {
        if i >= n {
            return 1;
        }
        let key = (i, cur, t as u8);
        if let Some(&v) = memo.get(&key) {
            return v;
        }

        let v = 1u32 << (s[i] - b'a');
        let nxt = cur | v;
        let mut ans = if nxt.count_ones() > k {
            dfs(i + 1, v, t, s, n, k, memo) + 1
        } else {
            dfs(i + 1, nxt, t, s, n, k, memo)
        };

        if t {
            for j in 0..26u32 {
                let nxt = cur | (1 << j);
                let candidate = if nxt.count_ones() > k {
                    dfs(i + 1, 1 << j, false, s, n, k, memo) + 1
                } else {
                    dfs(i + 1, nxt, false, s, n, k, memo)
                };
                ans = ans.max(candidate);
            }
        }

        memo.insert(key, ans);
        ans
    }

    let mut memo = HashMap::new();
    dfs(0, 0, true, s, n, k, &mut memo)
}

fn main() {
    println!("{}", max_partitions_after_operations("accca".into(), 2));
    println!("{}", max_partitions_after_operations("aabaab".into(), 3));
    println!("{}", max_partitions_after_operations("xxyz".into(), 1));
}

#[cfg(test)]
mod tests {
    use super::max_partitions_after_operations;

    #[test]
    fn example_one() {
        assert_eq!(max_partitions_after_operations("accca".into(), 2), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_partitions_after_operations("aabaab".into(), 3), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(max_partitions_after_operations("xxyz".into(), 1), 4);
    }
}
