/// LeetCode #3930 - Power Update After K-th Largest Insertion II
const MOD: i64 = 1_000_000_007;

fn mod_pow(mut base: i64, mut exp: i64) -> i64 {
    let mut result = 1i64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % MOD;
        }
        base = base * base % MOD;
        exp >>= 1;
    }
    result
}

fn bit_add(bit: &mut [i32], mut i: usize, delta: i32) {
    while i < bit.len() {
        bit[i] += delta;
        i += i & i.wrapping_neg();
    }
}

/// Smallest 1-based index whose prefix sum is at least `k`.
fn bit_kth(bit: &[i32], mut k: i32) -> usize {
    let mut idx = 0usize;
    let mut step = 1usize;
    while step << 1 < bit.len() {
        step <<= 1;
    }
    while step > 0 {
        let nxt = idx + step;
        if nxt < bit.len() && bit[nxt] < k {
            idx = nxt;
            k -= bit[nxt];
        }
        step >>= 1;
    }
    idx + 1
}

fn power_update(nums: Vec<i32>, mut p: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut vals = nums.clone();
    for q in &queries {
        vals.push(q[0]);
    }
    vals.sort_unstable();
    vals.dedup();
    let mut bit = vec![0i32; vals.len() + 1];
    let mut size = 0i32;
    for &x in &nums {
        let rank = vals.binary_search(&x).unwrap() + 1;
        bit_add(&mut bit, rank, 1);
        size += 1;
    }
    let mut ans = Vec::with_capacity(queries.len());
    for q in queries {
        let rank = vals.binary_search(&q[0]).unwrap() + 1;
        bit_add(&mut bit, rank, 1);
        size += 1;
        let pos = size - q[1] + 1;
        let x = vals[bit_kth(&bit, pos) - 1] as i64;
        p = mod_pow(p as i64, x) as i32;
        ans.push(p);
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        power_update(vec![2], 4, vec![vec![3, 1], vec![1, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::power_update;

    #[test]
    fn example1() {
        assert_eq!(
            power_update(vec![2], 4, vec![vec![3, 1], vec![1, 2]]),
            vec![64, 4096]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            power_update(vec![7, 5], 6, vec![vec![4, 3], vec![7, 2]]),
            vec![1296, 220296870]
        );
    }
}
