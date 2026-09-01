/// LeetCode #3539 - Find Sum of Array Product of Magical Sequences
use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

fn mod_pow(mut a: i64, mut e: i32) -> i64 {
    let mut r = 1i64;
    a %= MOD;
    while e > 0 {
        if e & 1 == 1 {
            r = r * a % MOD;
        }
        a = a * a % MOD;
        e >>= 1;
    }
    r
}

fn magical_sum(m: i32, k: i32, nums: Vec<i32>) -> i32 {
    let mx = 30usize;
    let mut fact = vec![1i64; mx + 1];
    let mut inv_fact = vec![1i64; mx + 1];
    for i in 1..=mx {
        fact[i] = fact[i - 1] * i as i64 % MOD;
        inv_fact[i] = mod_pow(fact[i], (MOD - 2) as i32);
    }
    let comb = |a: usize, b: usize| -> i64 {
        if b > a {
            0
        } else {
            fact[a] * inv_fact[b] % MOD * inv_fact[a - b] % MOD
        }
    };
    let mut memo: HashMap<(usize, usize, i32, usize), i64> = HashMap::new();
    fn dfs(
        i: usize,
        j: usize,
        k: i32,
        st: usize,
        nums: &[i32],
        comb: &dyn Fn(usize, usize) -> i64,
        memo: &mut HashMap<(usize, usize, i32, usize), i64>,
    ) -> i64 {
        if k < 0 || (i == nums.len() && j > 0) {
            return 0;
        }
        if let Some(&v) = memo.get(&(i, j, k, st)) {
            return v;
        }
        if i == nums.len() {
            let mut kk = k;
            let mut s = st;
            while s > 0 {
                kk -= (s & 1) as i32;
                s >>= 1;
            }
            let v = if kk == 0 { 1 } else { 0 };
            memo.insert((i, j, k, st), v);
            return v;
        }
        let mut res = 0i64;
        for t in 0..=j {
            let nt = t + st;
            let p = mod_pow(nums[i] as i64, t as i32);
            let nk = k - (nt & 1) as i32;
            res = (res + comb(j, t) * p % MOD * dfs(i + 1, j - t, nk, nt >> 1, nums, comb, memo)) % MOD;
        }
        memo.insert((i, j, k, st), res);
        res
    }
    dfs(0, m as usize, k, 0, &nums, &comb, &mut memo) as i32
}

fn main() {
    println!("{}", magical_sum(5, 5, vec![1, 10, 100, 10000, 1000000]));
}

#[cfg(test)]
mod tests {
    use super::magical_sum;

    #[test]
    fn example1() {
        assert_eq!(magical_sum(5, 5, vec![1, 10, 100, 10000, 1000000]), 991600007);
    }

    #[test]
    fn example2() {
        assert_eq!(magical_sum(2, 2, vec![5, 4, 3, 2, 1]), 170);
    }

    #[test]
    fn example3() {
        assert_eq!(magical_sum(1, 1, vec![28]), 28);
    }
}
