/// LeetCode #3765 - Complete Prime Number
fn complete_prime(num: i32) -> bool {
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
    let s = num.to_string();
    let b = s.as_bytes();
    let mut x = 0i64;
    for &c in b {
        x = x * 10 + (c - b'0') as i64;
        if !is_prime(x) {
            return false;
        }
    }
    x = 0;
    let mut p = 1i64;
    for &c in b.iter().rev() {
        x = p * (c - b'0') as i64 + x;
        p *= 10;
        if !is_prime(x) {
            return false;
        }
    }
    true
}

fn main() {
    println!("{}", complete_prime(23));
}

#[cfg(test)]
mod tests {
    use super::complete_prime;

    #[test]
    fn example1() {
        assert!(complete_prime(23));
    }

    #[test]
    fn example2() {
        assert!(!complete_prime(39));
    }

    #[test]
    fn example3() {
        assert!(complete_prime(7));
    }
}
