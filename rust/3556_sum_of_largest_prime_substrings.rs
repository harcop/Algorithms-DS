/// LeetCode #3556 - Sum of Largest Prime Substrings
fn is_prime(x: i64) -> bool {
    if x < 2 {
        return false;
    }
    let mut i = 2i64;
    while i * i <= x {
        if x % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn sum_of_largest_primes(s: String) -> i64 {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut st = std::collections::HashSet::new();
    for i in 0..n {
        let mut x = 0i64;
        for j in i..n {
            x = x * 10 + (bytes[j] - b'0') as i64;
            if is_prime(x) {
                st.insert(x);
            }
        }
    }
    let mut nums: Vec<i64> = st.into_iter().collect();
    nums.sort_unstable();
    nums.into_iter().rev().take(3).sum()
}

fn main() {
    println!("{}", sum_of_largest_primes("12234".into()));
}

#[cfg(test)]
mod tests {
    use super::sum_of_largest_primes;

    #[test]
    fn example1() {
        assert_eq!(sum_of_largest_primes("12234".into()), 1469);
    }

    #[test]
    fn example2() {
        assert_eq!(sum_of_largest_primes("111".into()), 11);
    }
}
