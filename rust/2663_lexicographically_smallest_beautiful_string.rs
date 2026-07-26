/// LeetCode #2663 - Lexicographically Smallest Beautiful String
fn smallest_beautiful_string(s: String, k: i32) -> String {
    let n = s.len();
    let mut cs: Vec<u8> = s.into_bytes();
    let k = k as u8;
    for i in (0..n).rev() {
        let p = cs[i] - b'a' + 1;
        for j in p..k {
            let c = b'a' + j;
            if (i > 0 && cs[i - 1] == c) || (i > 1 && cs[i - 2] == c) {
                continue;
            }
            cs[i] = c;
            for l in (i + 1)..n {
                for m in 0..k {
                    let c = b'a' + m;
                    if (l > 0 && cs[l - 1] == c) || (l > 1 && cs[l - 2] == c) {
                        continue;
                    }
                    cs[l] = c;
                    break;
                }
            }
            return String::from_utf8(cs).unwrap();
        }
    }
    String::new()
}

fn main() {
    println!("{}", smallest_beautiful_string("abcz".into(), 26));
}

#[cfg(test)]
mod tests {
    use super::smallest_beautiful_string;

    #[test]
    fn example_one() {
        assert_eq!(smallest_beautiful_string("abcz".into(), 26), "abda");
    }

    #[test]
    fn example_two() {
        assert_eq!(smallest_beautiful_string("dc".into(), 4), "");
    }
}
