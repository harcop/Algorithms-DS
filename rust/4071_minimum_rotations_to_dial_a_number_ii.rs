/// LeetCode #4071 - Minimum Rotations to Dial a Number II
fn min_rotations(_n: i32, s: String) -> i32 {
    fn dist(a: i32, b: i32) -> i32 {
        let d = (a - b).abs();
        d.min(10 - d)
    }

    let digits: Vec<i32> = s.bytes().map(|b| (b - b'0') as i32).collect();
    let n = digits.len();
    let mut suffix_pairs = vec![0i32; n + 1];
    for i in (0..n.saturating_sub(1)).rev() {
        suffix_pairs[i] = suffix_pairs[i + 1] + dist(digits[i], digits[i + 1]);
    }

    let mut ans = i32::MAX;
    let mut prefix = 0i32;
    let mut from = 0i32;
    for k in 0..n {
        ans = ans.min(prefix + dist(from, digits[n - 1]) + suffix_pairs[k]);
        prefix += dist(from, digits[k]);
        from = digits[k];
    }
    ans
}

fn main() {
    println!("{}", min_rotations(4, "1502".to_string()));
}

#[cfg(test)]
mod tests {
    use super::min_rotations;

    #[test]
    fn example1() {
        assert_eq!(min_rotations(4, "1502".to_string()), 9);
    }

    #[test]
    fn example2() {
        assert_eq!(min_rotations(4, "2916".to_string()), 12);
    }

    #[test]
    fn example3() {
        assert_eq!(min_rotations(4, "4219".to_string()), 6);
    }
}
