/// LeetCode #3456 - Find Special Substring of Length K
fn has_special_substring(s: String, k: i32) -> bool {
    let s = s.as_bytes();
    let n = s.len();
    let k = k as usize;
    let mut l = 0;
    while l < n {
        let mut r = l;
        while r < n && s[r] == s[l] {
            r += 1;
        }
        if r - l == k {
            return true;
        }
        l = r;
    }
    false
}

fn main() {
    println!("{}", has_special_substring("aaabaaa".into(), 3));
}

#[cfg(test)]
mod tests {
    use super::has_special_substring;

    #[test]
    fn example1() {
        assert!(has_special_substring("aaabaaa".into(), 3));
    }

    #[test]
    fn example2() {
        assert!(!has_special_substring("abc".into(), 2));
    }
}
