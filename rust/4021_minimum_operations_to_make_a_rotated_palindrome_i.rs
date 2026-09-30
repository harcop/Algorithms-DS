/// LeetCode #4021 - Minimum Operations to Make a Rotated Palindrome I
fn min_operations(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut ans = i32::MAX;
    for k in 0..n {
        let mut t = k as i32;
        let mut i = 0usize;
        let mut j = n - 1;
        while i < j {
            let x = s[(i + k) % n] - b'a';
            let y = s[(j + k) % n] - b'a';
            let d = (x as i32 - y as i32).abs();
            t += d.min(26 - d);
            i += 1;
            j -= 1;
        }
        ans = ans.min(t);
    }
    ans
}

fn main() {
    println!("{}", min_operations("abc".to_string()));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations("abc".to_string()), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations("yb".to_string()), 3);
    }
}
