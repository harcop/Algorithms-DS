/// LeetCode #2938 - Separate Black and White Balls
fn minimum_steps(s: String) -> i64 {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut ans = 0i64;
    let mut cnt = 0i64;
    for i in (0..n).rev() {
        if bytes[i] == b'1' {
            cnt += 1;
            ans += (n - i) as i64 - cnt;
        }
    }
    ans
}

fn main() {
    println!("{}", minimum_steps("101".into()));
}

#[cfg(test)]
mod tests {
    use super::minimum_steps;

    #[test]
    fn example_one() {
        assert_eq!(minimum_steps("101".into()), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_steps("100".into()), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_steps("0111".into()), 0);
    }
}
