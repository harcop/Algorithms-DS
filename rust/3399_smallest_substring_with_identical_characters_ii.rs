/// LeetCode #3399 - Smallest Substring With Identical Characters II
fn min_length(s: String, num_ops: i32) -> i32 {
    let s = s.into_bytes();
    let n = s.len();
    let check = |m: usize| -> bool {
        let mut cnt = 0i32;
        if m == 1 {
            let mut c = 0i32;
            for (i, &ch) in s.iter().enumerate() {
                let expect = if i % 2 == 0 { b'0' } else { b'1' };
                if ch == expect {
                    c += 1;
                }
            }
            cnt = c.min(n as i32 - c);
        } else {
            let mut k = 0usize;
            for i in 0..n {
                k += 1;
                if i == n - 1 || s[i] != s[i + 1] {
                    cnt += (k / (m + 1)) as i32;
                    k = 0;
                }
            }
        }
        cnt <= num_ops
    };
    let mut lo = 1;
    let mut hi = n;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if check(mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo as i32
}

fn main() {
    println!("{}", min_length("000001".into(), 1));
}

#[cfg(test)]
mod tests {
    use super::min_length;

    #[test]
    fn example1() {
        assert_eq!(min_length("000001".into(), 1), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(min_length("0000".into(), 2), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(min_length("0101".into(), 0), 1);
    }
}
