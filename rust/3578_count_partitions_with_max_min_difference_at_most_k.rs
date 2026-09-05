/// LeetCode #3578 - Count Partitions With Max-Min Difference at Most K
use std::collections::BTreeMap;

fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let n = nums.len();
    let mut f = vec![0; n + 1];
    let mut g = vec![0; n + 1];
    f[0] = 1;
    g[0] = 1;
    let mut sl = BTreeMap::new();
    let mut l = 1usize;
    for r in 1..=n {
        let x = nums[r - 1];
        *sl.entry(x).or_insert(0) += 1;
        while sl.keys().next_back().copied().unwrap() - sl.keys().next().copied().unwrap() > k {
            let val = nums[l - 1];
            if let Some(cnt) = sl.get_mut(&val) {
                *cnt -= 1;
                if *cnt == 0 {
                    sl.remove(&val);
                }
            }
            l += 1;
        }
        f[r] = (g[r - 1] - if l >= 2 { g[l - 2] } else { 0 } + MOD) % MOD;
        g[r] = (g[r - 1] + f[r]) % MOD;
    }
    f[n]
}

fn main() {
    println!("{}", count_partitions(vec![9, 4, 1, 3, 7], 4));
}

#[cfg(test)]
mod tests {
    use super::count_partitions;

    #[test]
    fn example1() {
        assert_eq!(count_partitions(vec![9, 4, 1, 3, 7], 4), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(count_partitions(vec![3, 3, 4], 0), 2);
    }
}
