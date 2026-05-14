/// LeetCode #762 - Prime Number of Set Bits in Binary Representation
fn is_prime(x: i32) -> bool {
    if x < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= x {
        if x % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

fn count_prime_set_bits(left: i32, right: i32) -> i32 {
    let mut ans = 0i32;
    for x in left..=right {
        let c = x.count_ones() as i32;
        if is_prime(c) {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", count_prime_set_bits(6, 10));
}

#[cfg(test)]
mod tests {
    use super::count_prime_set_bits;

    #[test]
    fn example_one() {
        assert_eq!(count_prime_set_bits(6, 10), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_prime_set_bits(10, 15), 5);
    }
}
