/// LeetCode #306 - Additive Number
fn is_additive_number(num: String) -> bool {
    let b = num.as_bytes();
    let n = b.len();
    for i in 1..=n / 2 {
        for j in i + 1..n {
            if b[0] == b'0' && i > 1 {
                continue;
            }
            if b[i] == b'0' && j > i + 1 {
                continue;
            }
            let mut a = parse(&b[..i]);
            let mut b2 = parse(&b[i..j]);
            let mut k = j;
            let mut ok = true;
            while k < n {
                let c = a + b2;
                let s = c.to_string();
                let t = s.as_bytes();
                if k + t.len() > n || &b[k..k + t.len()] != t {
                    ok = false;
                    break;
                }
                k += t.len();
                a = b2;
                b2 = c;
            }
            if ok && k == n {
                return true;
            }
        }
    }
    false
}

fn parse(b: &[u8]) -> i64 {
    String::from_utf8_lossy(b).parse().unwrap()
}

fn main() {
    println!("{}", is_additive_number("112358".into()));
}

#[cfg(test)]
mod tests {
    use super::is_additive_number;

    #[test]
    fn example_one() {
        assert!(is_additive_number("112358".into()));
    }

    #[test]
    fn example_two() {
        assert!(!is_additive_number("102".into()));
    }
}
