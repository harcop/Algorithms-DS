/// LeetCode #3144 - Minimum Substring Partition of Equal Character Frequency
use std::collections::HashMap;

fn minimum_substrings_in_partition(s: String) -> i32 {
    let n = s.len();
    let bytes = s.as_bytes();
    let mut memo = vec![-1i32; n + 1];
    fn dfs(
        i: usize,
        n: usize,
        bytes: &[u8],
        memo: &mut [i32],
    ) -> i32 {
        if i >= n {
            return 0;
        }
        if memo[i] != -1 {
            return memo[i];
        }
        let mut cnt: HashMap<u8, i32> = HashMap::new();
        let mut freq: HashMap<i32, i32> = HashMap::new();
        let mut ans = (n - i) as i32;
        for j in i..n {
            let c = bytes[j];
            if let Some(&old) = cnt.get(&c) {
                let e = freq.get_mut(&old).unwrap();
                *e -= 1;
                if *e == 0 {
                    freq.remove(&old);
                }
            }
            let new = cnt.get(&c).copied().unwrap_or(0) + 1;
            cnt.insert(c, new);
            *freq.entry(new).or_insert(0) += 1;
            if freq.len() == 1 {
                ans = ans.min(1 + dfs(j + 1, n, bytes, memo));
            }
        }
        memo[i] = ans;
        ans
    }
    dfs(0, n, bytes, &mut memo)
}

fn main() {
    println!("{}", minimum_substrings_in_partition("fabccddg".into()));
}

#[cfg(test)]
mod tests {
    use super::minimum_substrings_in_partition;

    #[test]
    fn example1() {
        assert_eq!(minimum_substrings_in_partition("fabccddg".into()), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_substrings_in_partition("abababaccddb".into()), 2);
    }
}
