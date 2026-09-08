/// LeetCode #3628 - Maximum Number of Subsequences After One Inserting
fn calc(s: &str, t: &[u8; 2]) -> i64 {
    let mut cnt = 0i64;
    let mut a = 0i64;
    for c in s.bytes() {
        if c == t[1] {
            cnt += a;
        }
        if c == t[0] {
            a += 1;
        }
    }
    cnt
}

fn num_of_subsequences(s: String) -> i64 {
    let mut l = 0i64;
    let mut r = s.bytes().filter(|&c| c == b'T').count() as i64;
    let mut ans = 0i64;
    let mut mx = 0i64;
    for c in s.bytes() {
        if c == b'T' {
            r -= 1;
        }
        if c == b'C' {
            ans += l * r;
        }
        if c == b'L' {
            l += 1;
        }
        mx = mx.max(l * r);
    }
    mx = mx.max(calc(&s, b"LC")).max(calc(&s, b"CT"));
    ans + mx
}

fn main() {
    println!("{}", num_of_subsequences("LMCT".into()));
}

#[cfg(test)]
mod tests {
    use super::num_of_subsequences;

    #[test]
    fn example1() {
        assert_eq!(num_of_subsequences("LMCT".into()), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(num_of_subsequences("LCCT".into()), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(num_of_subsequences("L".into()), 0);
    }
}
