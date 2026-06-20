/// LeetCode #1994 - The Number of Good Subsets
const MOD: i64 = 1_000_000_007;
const PRIMES: [i32; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

fn pow_mod(mut base: i64, mut exp: i64) -> i64 {
    let mut ans = 1i64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            ans = ans * base % MOD;
        }
        base = base * base % MOD;
        exp >>= 1;
    }
    ans
}

fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
    let mut cnt = [0i64; 31];
    for x in nums {
        cnt[x as usize] += 1;
    }

    let n = PRIMES.len();
    let mut f = vec![0i64; 1 << n];
    f[0] = pow_mod(2, cnt[1]);

    for x in 2..31i32 {
        if cnt[x as usize] == 0 || x % 4 == 0 || x % 9 == 0 || x % 25 == 0 {
            continue;
        }
        let mut mask = 0usize;
        for (i, &p) in PRIMES.iter().enumerate() {
            if x % p == 0 {
                mask |= 1 << i;
            }
        }
        for state in (1..(1 << n)).rev() {
            if state & mask == mask {
                f[state] = (f[state] + cnt[x as usize] * f[state ^ mask]) % MOD;
            }
        }
    }

    ((1..(1 << n)).map(|i| f[i]).sum::<i64>() % MOD) as i32
}

fn main() {
    println!("{}", number_of_good_subsets(vec![1, 2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::number_of_good_subsets;

    #[test]
    fn example_one() {
        assert_eq!(number_of_good_subsets(vec![1, 2, 3, 4]), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_good_subsets(vec![4, 2, 3, 15]), 5);
    }
}
