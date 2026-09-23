/// LeetCode #3896 - Minimum Operations to Transform Array Into Alternating Prime

const SIEVE_LIMIT: usize = 200_001;

fn build_primes() -> Vec<bool> {
    let mut is_prime = vec![true; SIEVE_LIMIT];
    is_prime[0] = false;
    is_prime[1] = false;
    for p in 2..SIEVE_LIMIT {
        if !is_prime[p] {
            continue;
        }
        let mut q = p * p;
        while q < SIEVE_LIMIT {
            is_prime[q] = false;
            q += p;
        }
    }
    is_prime
}

fn next_prime_at_least(is_prime: &[bool], x: i32) -> i32 {
    let mut v = x.max(2) as usize;
    while v < SIEVE_LIMIT && !is_prime[v] {
        v += 1;
    }
    v as i32
}

fn minimum_operations(nums: Vec<i32>) -> i64 {
    let is_prime = build_primes();
    let mut ops = 0i64;
    for (i, &x) in nums.iter().enumerate() {
        if i % 2 == 0 {
            let p = next_prime_at_least(&is_prime, x);
            ops += (p - x) as i64;
        } else if is_prime.get(x as usize).copied().unwrap_or(false) {
            ops += if x == 2 { 2 } else { 1 };
        }
    }
    ops
}

fn main() {
    println!("{}", minimum_operations(vec![1, 2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::minimum_operations;

    #[test]
    fn example1() {
        assert_eq!(minimum_operations(vec![1, 2, 3, 4]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_operations(vec![5, 6, 7, 8]), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(minimum_operations(vec![4, 4]), 1);
    }
}
