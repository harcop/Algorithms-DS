/// LeetCode #3610 - Minimum Number of Primes to Sum to Target
fn first_primes(m: usize) -> Vec<i32> {
    let mut primes = Vec::with_capacity(m);
    let mut x = 2;
    while primes.len() < m {
        let mut is_prime = true;
        for &p in &primes {
            if p * p > x {
                break;
            }
            if x % p == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(x);
        }
        x += 1;
    }
    primes
}

fn min_number_of_primes(n: i32, m: i32) -> i32 {
    let n = n as usize;
    let primes = first_primes(m as usize);
    const INF: i32 = 1_000_000_000;
    let mut f = vec![INF; n + 1];
    f[0] = 0;
    for x in primes {
        let x = x as usize;
        for i in x..=n {
            f[i] = f[i].min(f[i - x] + 1);
        }
    }
    if f[n] >= INF {
        -1
    } else {
        f[n]
    }
}

fn main() {
    println!("{}", min_number_of_primes(10, 2));
}

#[cfg(test)]
mod tests {
    use super::min_number_of_primes;

    #[test]
    fn example1() {
        assert_eq!(min_number_of_primes(10, 2), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(min_number_of_primes(15, 5), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(min_number_of_primes(7, 6), 1);
    }
}
