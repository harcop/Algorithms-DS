/// LeetCode #878 - Nth Magical Number
fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        let t = x % y;
        x = y;
        y = t;
    }
    x
}

fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = n as i64;
    let a = a as i64;
    let b = b as i64;
    let g = gcd(a, b);
    let lcm_ab = a * b / g;

    let count = |m: i64| m / a + m / b - m / lcm_ab;

    let mut lo = 1i64;
    let mut hi = n * a.min(b);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if count(mid) < n {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    (lo % MOD) as i32
}

fn main() {
    println!("{}", nth_magical_number(1, 2, 3));
}

#[cfg(test)]
mod tests {
    use super::nth_magical_number;

    #[test]
    fn example_one() {
        assert_eq!(nth_magical_number(1, 2, 3), 2);
    }
}
