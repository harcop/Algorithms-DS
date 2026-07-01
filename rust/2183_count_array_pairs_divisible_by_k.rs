/// LeetCode #2183 - Count Array Pairs Divisible by K
use std::collections::HashMap;

fn factorize(mut k: i32) -> Vec<(i32, i32)> {
    let mut factors = Vec::new();
    let mut d = 2i32;
    while d * d <= k {
        if k % d == 0 {
            let mut count = 0;
            while k % d == 0 {
                k /= d;
                count += 1;
            }
            factors.push((d, count));
        }
        d += 1;
    }
    if k > 1 {
        factors.push((k, 1));
    }
    factors
}

fn signature(mut x: i32, factors: &[(i32, i32)]) -> Vec<i32> {
    let mut vals = Vec::with_capacity(factors.len());
    for &(p, req) in factors {
        let mut count = 0i32;
        while x % p == 0 {
            x /= p;
            count += 1;
        }
        vals.push(count.min(req));
    }
    vals
}

fn count_pairs(nums: Vec<i32>, k: i32) -> i64 {
    let factors = factorize(k);
    let m = factors.len();
    let limits: Vec<i32> = factors.iter().map(|&(_, req)| req).collect();

    let mut keys = Vec::new();
    fn gen(pos: usize, cur: &mut Vec<i32>, limits: &[i32], keys: &mut Vec<Vec<i32>>) {
        if pos == limits.len() {
            keys.push(cur.clone());
            return;
        }
        for v in 0..=limits[pos] {
            cur.push(v);
            gen(pos + 1, cur, limits, keys);
            cur.pop();
        }
    }
    gen(0, &mut Vec::with_capacity(m), &limits, &mut keys);

    let mut freq: HashMap<Vec<i32>, i64> = HashMap::new();
    let mut ans = 0i64;

    for &num in &nums {
        let mut x = num;
        let mut need = vec![0i32; m];
        for (i, &(p, req)) in factors.iter().enumerate() {
            let mut count = 0i32;
            while x % p == 0 {
                x /= p;
                count += 1;
            }
            need[i] = (req - count).max(0);
        }

        for key in &keys {
            if key.iter().zip(need.iter()).all(|(a, b)| a >= b) {
                ans += freq.get(key).copied().unwrap_or(0);
            }
        }

        let sig = signature(num, &factors);
        *freq.entry(sig).or_insert(0) += 1;
    }

    ans
}

fn main() {
    println!("{}", count_pairs(vec![1, 2, 3, 4, 5], 2));
}

#[cfg(test)]
mod tests {
    use super::count_pairs;

    #[test]
    fn example_one() {
        assert_eq!(count_pairs(vec![1, 2, 3, 4, 5], 2), 7);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_pairs(vec![1, 2, 3, 4], 5), 0);
    }
}
