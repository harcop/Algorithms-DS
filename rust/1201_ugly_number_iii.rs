/// LeetCode #1201 - Ugly Number III
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
    let (a, b, c) = (a as i64, b as i64, c as i64);
    let ab = lcm(a, b);
    let ac = lcm(a, c);
    let bc = lcm(b, c);
    let abc = lcm(ab, c);
    let n = n as i64;
    let mut lo = 1i64;
    let mut hi = 2_000_000_000i64;
    while lo < hi {
        let mid = (lo + hi) / 2;
        let cnt = mid / a + mid / b + mid / c - mid / ab - mid / ac - mid / bc + mid / abc;
        if cnt >= n {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo as i32
}

fn main() {
    println!("{}", nth_ugly_number(3, 2, 3, 5));
}

#[cfg(test)]
mod tests {
    use super::nth_ugly_number;

    #[test]
    fn example_one() {
        assert_eq!(nth_ugly_number(3, 2, 3, 5), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(nth_ugly_number(5, 2, 11, 13), 10);
    }

    #[test]
    fn example_three() {
        assert_eq!(nth_ugly_number(1000000000, 2, 217983653, 336916467), 1999999984);
    }
}
