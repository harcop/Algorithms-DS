/// LeetCode #2523 - Closest Prime Numbers in Range
fn closest_primes(left: i32, right: i32) -> Vec<i32> {
    let right = right as usize;
    let left = left as usize;
    let mut is_prime = vec![true; right + 1];
    if right >= 1 {
        is_prime[1] = false;
    }
    for i in 2..=((right as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in (i * i..=right).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    let mut primes = Vec::new();
    for p in left..=right {
        if is_prime[p] {
            primes.push(p as i32);
        }
    }
    if primes.len() < 2 {
        return vec![-1, -1];
    }

    let mut ans = vec![primes[0], primes[1]];
    let mut best = ans[1] - ans[0];
    for i in 1..primes.len() - 1 {
        let gap = primes[i + 1] - primes[i];
        if gap < best {
            best = gap;
            ans = vec![primes[i], primes[i + 1]];
        }
    }
    ans
}

fn main() {
    println!("{:?}", closest_primes(10, 19));
}

#[cfg(test)]
mod tests {
    use super::closest_primes;

    #[test]
    fn example_one() {
        assert_eq!(closest_primes(10, 19), vec![11, 13]);
    }

    #[test]
    fn example_two() {
        assert_eq!(closest_primes(4, 6), vec![-1, -1]);
    }
}
