/// LeetCode #2572 - Count the Number of Square-Free Subsets
const MOD: i64 = 1_000_000_007;
const PRIMES: [i32; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

fn square_free_subsets(nums: Vec<i32>) -> i32 {
    let mut cnt = [0i64; 31];
    for x in nums {
        cnt[x as usize] += 1;
    }

    let n = PRIMES.len();
    let mut f = vec![0i64; 1 << n];
    f[0] = 1;
    for _ in 0..cnt[1] {
        f[0] = f[0] * 2 % MOD;
    }

    for x in 2..31 {
        if cnt[x] == 0 || x % 4 == 0 || x % 9 == 0 || x % 25 == 0 {
            continue;
        }
        let mut mask = 0usize;
        for (i, &p) in PRIMES.iter().enumerate() {
            if x as i32 % p == 0 {
                mask |= 1 << i;
            }
        }
        for state in (1..(1 << n)).rev() {
            if state & mask == mask {
                f[state] = (f[state] + cnt[x] * f[state ^ mask]) % MOD;
            }
        }
    }

    let mut ans = MOD - 1;
    for v in f {
        ans = (ans + v) % MOD;
    }
    ans as i32
}

fn main() {
    println!("{}", square_free_subsets(vec![3, 4, 4, 5]));
}

#[cfg(test)]
mod tests {
    use super::square_free_subsets;

    #[test]
    fn example_one() {
        assert_eq!(square_free_subsets(vec![3, 4, 4, 5]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(square_free_subsets(vec![1]), 1);
    }
}
