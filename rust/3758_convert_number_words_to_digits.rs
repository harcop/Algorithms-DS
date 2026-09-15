/// LeetCode #3758 - Convert Number Words to Digits (premium)
fn convert_number(s: String) -> String {
    let d = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut i = 0usize;
    let mut ans = String::new();
    while i < n {
        let mut matched = false;
        for (j, t) in d.iter().enumerate() {
            let m = t.len();
            if i + m <= n && &bytes[i..i + m] == t.as_bytes() {
                ans.push((b'0' + j as u8) as char);
                i += m;
                matched = true;
                break;
            }
        }
        if !matched {
            i += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", convert_number("onefourthree".into()));
}

#[cfg(test)]
mod tests {
    use super::convert_number;

    #[test]
    fn example1() {
        assert_eq!(convert_number("onefourthree".into()), "143");
    }

    #[test]
    fn example2() {
        assert_eq!(convert_number("ninexsix".into()), "96");
    }

    #[test]
    fn example3() {
        assert_eq!(convert_number("zeero".into()), "");
    }

    #[test]
    fn example4() {
        assert_eq!(convert_number("tw".into()), "");
    }
}
