/// LeetCode #3474 - Lexicographically Smallest Generated String
fn generate_string(str1: String, str2: String) -> String {
    let s1 = str1.as_bytes();
    let s2 = str2.as_bytes();
    let n = s1.len();
    let m = s2.len();
    let mut ans = vec![b'a'; n + m - 1];
    let mut fixed = vec![false; n + m - 1];
    for i in 0..n {
        if s1[i] != b'T' {
            continue;
        }
        for j in 0..m {
            if fixed[i + j] && ans[i + j] != s2[j] {
                return String::new();
            }
            ans[i + j] = s2[j];
            fixed[i + j] = true;
        }
    }
    loop {
        let mut changed = false;
        for i in 0..n {
            if s1[i] != b'F' || &ans[i..i + m] != s2 {
                continue;
            }
            let mut ok = false;
            for j in (0..m).rev() {
                if !fixed[i + j] {
                    ans[i + j] = b'b';
                    fixed[i + j] = true;
                    ok = true;
                    changed = true;
                    break;
                }
            }
            if !ok {
                return String::new();
            }
        }
        if !changed {
            break;
        }
    }
    for i in 0..n {
        let eq = &ans[i..i + m] == s2;
        if (s1[i] == b'T') != eq {
            return String::new();
        }
    }
    String::from_utf8(ans).unwrap()
}

fn main() {
    println!("{}", generate_string("TFTF".into(), "ab".into()));
}

#[cfg(test)]
mod tests {
    use super::generate_string;

    #[test]
    fn example1() {
        assert_eq!(generate_string("TFTF".into(), "ab".into()), "ababa");
    }

    #[test]
    fn example2() {
        assert_eq!(generate_string("TFTF".into(), "abc".into()), "");
    }

    #[test]
    fn example3() {
        assert_eq!(generate_string("F".into(), "d".into()), "a");
    }
}
