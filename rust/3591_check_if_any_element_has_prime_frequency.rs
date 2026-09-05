/// LeetCode #3591 - Check if Any Element Has Prime Frequency
use std::collections::HashMap;

fn is_prime(x: i32) -> bool {
    if x < 2 {
        return false;
    }
    let mut i = 2;
    while i * i <= x {
        if x % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn check_prime_frequency(nums: Vec<i32>) -> bool {
    let mut cnt = HashMap::new();
    for x in nums {
        *cnt.entry(x).or_insert(0) += 1;
    }
    cnt.values().any(|&x| is_prime(x))
}

fn main() {
    println!("{}", check_prime_frequency(vec![1, 2, 3, 4, 5, 4]));
}

#[cfg(test)]
mod tests {
    use super::check_prime_frequency;

    #[test]
    fn example1() {
        assert_eq!(check_prime_frequency(vec![1, 2, 3, 4, 5, 4]), true);
    }

    #[test]
    fn example2() {
        assert_eq!(check_prime_frequency(vec![1, 2, 3, 4, 5]), false);
    }

    #[test]
    fn example3() {
        assert_eq!(check_prime_frequency(vec![2, 2, 2, 4, 4]), true);
    }
}
