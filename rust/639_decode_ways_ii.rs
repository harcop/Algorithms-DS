/// LeetCode #639 - Decode Ways II
fn num_decodings(s: String) -> i32 {
    const MOD: i64 = 1_000_000_007;
    fn ways1(c: u8) -> i64 {
        match c {
            b'*' => 9,
            b'0' => 0,
            _ => 1,
        }
    }
    fn ways2(a: u8, b: u8) -> i64 {
        match (a, b) {
            (b'*', b'*') => 15,
            (b'*', b'0') => 2,
            (b'*', x) if x <= b'6' => 2,
            (b'*', _) => 1,
            (b'1', b'*') => 9,
            (b'2', b'*') => 6,
            (b'1', _) => 1,
            (b'2', x) if x <= b'6' => 1,
            _ => 0,
        }
    }
    let s = s.as_bytes();
    let n = s.len();
    let mut dp0 = 1i64;
    let mut dp1 = ways1(s[0]);
    for i in 1..n {
        let cur = (dp1 * ways1(s[i]) + dp0 * ways2(s[i - 1], s[i])) % MOD;
        dp0 = dp1;
        dp1 = cur;
    }
    dp1 as i32
}

fn main() {
    println!("{}", num_decodings("*".into()));
}

#[cfg(test)]
mod tests {
    use super::num_decodings;

    #[test]
    fn example_one() {
        assert_eq!(num_decodings("*".into()), 9);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_decodings("1*".into()), 18);
    }

    #[test]
    fn example_three() {
        assert_eq!(num_decodings("2*".into()), 15);
    }
}
