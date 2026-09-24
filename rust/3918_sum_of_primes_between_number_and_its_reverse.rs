/// LeetCode #3918 - Sum of Primes Between Number and Its Reverse
fn sum_of_primes(n: i32) -> i32 {
    let mut rev = 0i32;
    let mut x = n;
    while x > 0 {
        rev = rev * 10 + x % 10;
        x /= 10;
    }
    let (lo, hi) = if n < rev { (n, rev) } else { (rev, n) };
    let hi = hi as usize;
    let mut prime = vec![true; hi + 1];
    if hi >= 1 {
        prime[0] = false;
    }
    if hi >= 1 {
        prime[1] = false;
    }
    let mut i = 2usize;
    while i * i <= hi {
        if prime[i] {
            let mut j = i * i;
            while j <= hi {
                prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    let mut sum = 0i32;
    for v in lo.max(2)..=hi as i32 {
        if prime[v as usize] {
            sum += v;
        }
    }
    sum
}

fn main() {
    println!("{}", sum_of_primes(13));
}

#[cfg(test)]
mod tests {
    use super::sum_of_primes;

    #[test]
    fn example1() {
        assert_eq!(sum_of_primes(13), 132);
    }

    #[test]
    fn example2() {
        assert_eq!(sum_of_primes(10), 17);
    }

    #[test]
    fn example3() {
        assert_eq!(sum_of_primes(8), 0);
    }
}
