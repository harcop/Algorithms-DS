/// LeetCode #3757 - Number of Effective Subsequences
fn number_of_effective_subsequences(nums: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = nums.len() as i64;
    let mut g = 0i32;
    for &x in &nums {
        g |= x;
    }
    let mut bit_list = Vec::new();
    for b in 0..21 {
        if g & (1 << b) != 0 {
            bit_list.push(b);
        }
    }
    let bsz = bit_list.len();
    if bsz == 0 {
        return 0;
    }
    let mut f = vec![0i64; 1 << bsz];
    for &x in &nums {
        let mut v = 0usize;
        for (i, &b) in bit_list.iter().enumerate() {
            if x & (1 << b) != 0 {
                v |= 1 << i;
            }
        }
        f[v] += 1;
    }
    let mut sub = f;
    for i in 0..bsz {
        for mask in 0..(1 << bsz) {
            if mask & (1 << i) != 0 {
                sub[mask] += sub[mask ^ (1 << i)];
            }
        }
    }
    let all = (1 << bsz) - 1;
    let mut ans = 0i64;
    for mask in 1..=all {
        let none = sub[all ^ mask];
        let covered = n - none;
        let ways = mod_pow(2, n - covered, MOD);
        if mask.count_ones() % 2 == 1 {
            ans = (ans + ways) % MOD;
        } else {
            ans = (ans - ways + MOD) % MOD;
        }
    }
    ans as i32
}

fn mod_pow(mut a: i64, mut e: i64, m: i64) -> i64 {
    let mut r = 1i64;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = r * a % m;
        }
        a = a * a % m;
        e >>= 1;
    }
    r
}

fn main() {
    println!("{}", number_of_effective_subsequences(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::number_of_effective_subsequences;

    #[test]
    fn example1() {
        assert_eq!(number_of_effective_subsequences(vec![1, 2, 3]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(number_of_effective_subsequences(vec![7, 4, 6]), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(number_of_effective_subsequences(vec![8, 8]), 1);
    }

    #[test]
    fn example4() {
        assert_eq!(number_of_effective_subsequences(vec![2, 2, 1]), 5);
    }
}
