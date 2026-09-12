/// LeetCode #3722 - Lexicographically Smallest String After Reverse
fn lex_smallest(s: String) -> String {
    let n = s.len();
    let bytes = s.as_bytes();
    let mut best = s.clone();
    for k in 1..=n {
        let mut a = bytes[..k].to_vec();
        a.reverse();
        a.extend_from_slice(&bytes[k..]);
        let cand = String::from_utf8(a).unwrap();
        if cand < best {
            best = cand;
        }
        let mut b = bytes[..n - k].to_vec();
        let mut tail = bytes[n - k..].to_vec();
        tail.reverse();
        b.extend(tail);
        let cand = String::from_utf8(b).unwrap();
        if cand < best {
            best = cand;
        }
    }
    best
}

fn main() {
    println!("{}", lex_smallest("dcab".into()));
}

#[cfg(test)]
mod tests {
    use super::lex_smallest;

    #[test]
    fn example1() {
        assert_eq!(lex_smallest("dcab".into()), "acdb");
    }

    #[test]
    fn example2() {
        assert_eq!(lex_smallest("abba".into()), "aabb");
    }

    #[test]
    fn example3() {
        assert_eq!(lex_smallest("zxy".into()), "xzy");
    }
}
