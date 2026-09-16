/// LeetCode #3770 - Largest Prime from Consecutive Prime Sum
fn largest_prime(n: i32) -> i32 {
    if n < 2 {
        return 0;
    }
    let mx = n as usize;
    let mut is_prime = vec![true; mx + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut primes = Vec::new();
    for i in 2..=mx {
        if is_prime[i] {
            primes.push(i);
            let mut j = i.saturating_mul(i);
            while j <= mx {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    let mut s = vec![0];
    let mut t = 0usize;
    for x in primes {
        t += x;
        if t > mx {
            break;
        }
        if is_prime[t] {
            s.push(t as i32);
        }
    }
    let i = s.partition_point(|&x| x <= n);
    s[i - 1]
}

fn main() {
    println!("{}", largest_prime(20));
}

#[cfg(test)]
mod tests {
    use super::largest_prime;

    #[test]
    fn example1() {
        assert_eq!(largest_prime(20), 17);
    }

    #[test]
    fn example2() {
        assert_eq!(largest_prime(2), 2);
    }
}
