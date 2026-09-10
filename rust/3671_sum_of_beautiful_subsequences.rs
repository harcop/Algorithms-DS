/// LeetCode #3671 - Sum of Beautiful Subsequences
const MOD: i64 = 1_000_000_007;

struct Fenwick {
    bit: Vec<i64>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Self {
            bit: vec![0; n + 2],
        }
    }
    fn add(&mut self, mut i: usize, val: i64) {
        i += 1;
        while i < self.bit.len() {
            self.bit[i] = (self.bit[i] + val) % MOD;
            i += i & (!i + 1);
        }
    }
    fn query(&self, mut i: isize) -> i64 {
        if i < 0 {
            return 0;
        }
        let mut i = (i + 1) as usize;
        let mut total = 0i64;
        while i > 0 {
            total = (total + self.bit[i]) % MOD;
            i -= i & (!i + 1);
        }
        total
    }
}

fn count_increasing(arr: &[i32]) -> i64 {
    if arr.is_empty() {
        return 0;
    }
    let mut vals: Vec<i32> = arr.to_vec();
    vals.sort_unstable();
    vals.dedup();
    let rank = |x: i32| -> usize { vals.binary_search(&x).unwrap() };
    let mut bit = Fenwick::new(vals.len());
    for &x in arr {
        let r = rank(x);
        let ways = (bit.query(r as isize - 1) + 1) % MOD;
        bit.add(r, ways);
    }
    bit.query(vals.len() as isize - 1)
}

fn total_beauty(nums: Vec<i32>) -> i32 {
    let mx = *nums.iter().max().unwrap() as usize;
    let mut lookup = vec![Vec::new(); mx + 1];
    for &x in &nums {
        let mut d = 1;
        while d * d <= x {
            if x % d == 0 {
                lookup[d as usize].push(x);
                if d * d != x {
                    lookup[(x / d) as usize].push(x);
                }
            }
            d += 1;
        }
    }
    let mut cnt = vec![0i64; mx + 1];
    let mut result = 0i64;
    for g in (1..=mx).rev() {
        cnt[g] = count_increasing(&lookup[g]);
        let mut ng = g * 2;
        while ng <= mx {
            cnt[g] = (cnt[g] - cnt[ng]).rem_euclid(MOD);
            ng += g;
        }
        result = (result + (g as i64) * cnt[g]) % MOD;
    }
    result as i32
}

fn main() {
    println!("{}", total_beauty(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::total_beauty;

    #[test]
    fn example1() {
        assert_eq!(total_beauty(vec![1, 2, 3]), 10);
    }

    #[test]
    fn example2() {
        assert_eq!(total_beauty(vec![4, 6]), 12);
    }
}
