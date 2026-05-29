/// LeetCode #1540 - Can Convert String In K Moves
fn can_convert_string(s: String, t: String, k: i32) -> bool {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut cnt = vec![0i32; 26];
    for i in 0..s.len() {
        let diff = (t[i] as i32 - s[i] as i32 + 26) % 26;
        if diff > 0 {
            cnt[diff as usize] += 1;
        }
    }
    for d in 1..26usize {
        if cnt[d] > 0 && d as i32 + 26 * (cnt[d] - 1) > k {
            return false;
        }
    }
    true
}

fn main() {
    println!("{}", can_convert_string("input".into(), "ouput".into(), 9));
}

#[cfg(test)]
mod tests {
    use super::can_convert_string;

    #[test]
    fn example_one() {
        assert!(can_convert_string("input".into(), "ouput".into(), 9));
    }

    #[test]
    fn example_two() {
        assert!(!can_convert_string("abc".into(), "bcd".into(), 10));
    }
}
