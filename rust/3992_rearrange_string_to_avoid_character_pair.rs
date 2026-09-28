/// LeetCode #3992 - Rearrange String to Avoid Character Pair
fn rearrange_string(s: String, x: char, y: char) -> String {
    let _ = x;
    let mut t = s.into_bytes();
    let y = y as u8;
    let mut i = 0;
    for j in 0..t.len() {
        if t[j] == y {
            t.swap(i, j);
            i += 1;
        }
    }
    String::from_utf8(t).unwrap()
}

fn main() {
    println!("{}", rearrange_string("aabc".into(), 'a', 'c'));
}

#[cfg(test)]
mod tests {
    use super::rearrange_string;

    fn is_valid(s: &str, t: &str, x: char, y: char) -> bool {
        let mut a: Vec<u8> = s.bytes().collect();
        let mut b: Vec<u8> = t.bytes().collect();
        a.sort_unstable();
        b.sort_unstable();
        if a != b {
            return false;
        }
        let mut seen_x = false;
        for c in t.chars() {
            if c == x {
                seen_x = true;
            }
            if c == y && seen_x {
                return false;
            }
        }
        true
    }

    #[test]
    fn example1() {
        let s = "aabc";
        let t = rearrange_string(s.into(), 'a', 'c');
        assert!(is_valid(s, &t, 'a', 'c'));
    }

    #[test]
    fn example2() {
        let s = "dcab";
        let t = rearrange_string(s.into(), 'd', 'b');
        assert!(is_valid(s, &t, 'd', 'b'));
    }

    #[test]
    fn example3() {
        let s = "axe";
        let t = rearrange_string(s.into(), 'o', 'x');
        assert!(is_valid(s, &t, 'o', 'x'));
    }
}
