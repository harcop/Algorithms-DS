/// LeetCode #204 - Count Primes
fn count_primes(n: i32) -> i32 {
    if n <= 2 {
        return 0;
    }
    let n = n as usize;
    let mut sieve = vec![true; n];
    sieve[0] = false;
    sieve[1] = false;
    let mut c = 0;
    for i in 2..n {
        if !sieve[i] {
            continue;
        }
        c += 1;
        let mut j = i * i;
        while j < n {
            sieve[j] = false;
            j += i;
        }
    }
    c
}

fn main() {
    println!("{}", count_primes(10));
}

#[cfg(test)]
mod tests {
    use super::count_primes;

    #[test]
    fn example_one() {
        assert_eq!(count_primes(10), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_primes(0), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(count_primes(1), 0);
    }
}
