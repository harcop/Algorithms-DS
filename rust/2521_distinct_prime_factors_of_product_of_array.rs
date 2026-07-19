/// LeetCode #2521 - Distinct Prime Factors of Product of Array
use std::collections::HashSet;

fn distinct_prime_factors(nums: Vec<i32>) -> i32 {
    let mut s = HashSet::new();
    for mut n in nums {
        let mut i = 2;
        while i <= n / i {
            if n % i == 0 {
                s.insert(i);
                while n % i == 0 {
                    n /= i;
                }
            }
            i += 1;
        }
        if n > 1 {
            s.insert(n);
        }
    }
    s.len() as i32
}

fn main() {
    println!("{}", distinct_prime_factors(vec![2, 4, 3, 7, 10, 6]));
}

#[cfg(test)]
mod tests {
    use super::distinct_prime_factors;

    #[test]
    fn example_one() {
        assert_eq!(distinct_prime_factors(vec![2, 4, 3, 7, 10, 6]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(distinct_prime_factors(vec![2, 4, 8, 16]), 1);
    }
}
