/// LeetCode #1234 - Replace the Substring for Balanced String
fn balanced_string(s: String) -> i32 {
    let b = s.as_bytes();
    let n = b.len();
    let mut cnt = [0i32; 4];
    let idx = |c: u8| -> usize {
        match c {
            b'Q' => 0,
            b'W' => 1,
            b'E' => 2,
            _ => 3,
        }
    };
    for &c in b {
        cnt[idx(c)] += 1;
    }
    let target = n as i32 / 4;
    if cnt.iter().all(|&x| x == target) {
        return 0;
    }
    let mut ans = n as i32;
    let mut l = 0usize;
    for r in 0..n {
        cnt[idx(b[r])] -= 1;
        while l <= r && cnt.iter().all(|&x| x <= target) {
            ans = ans.min((r - l + 1) as i32);
            cnt[idx(b[l])] += 1;
            l += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", balanced_string("QWER".into()));
}

#[cfg(test)]
mod tests {
    use super::balanced_string;

    #[test]
    fn example_one() {
        assert_eq!(balanced_string("QWER".into()), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(balanced_string("QQWE".into()), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(balanced_string("QQQW".into()), 2);
    }
}
