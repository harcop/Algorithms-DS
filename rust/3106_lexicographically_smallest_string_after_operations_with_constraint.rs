/// LeetCode #3106 - Lexicographically Smallest String After Operations With Constraint
fn get_smallest_string(s: String, mut k: i32) -> String {
    let mut cs: Vec<u8> = s.into_bytes();
    for i in 0..cs.len() {
        let c1 = cs[i];
        for c2 in b'a'..c1 {
            let d = (c1 - c2).min(26 - (c1 - c2)) as i32;
            if d <= k {
                cs[i] = c2;
                k -= d;
                break;
            }
        }
    }
    String::from_utf8(cs).unwrap()
}

fn main() {
    println!("{}", get_smallest_string("zbbz".into(), 3));
}

#[cfg(test)]
mod tests {
    use super::get_smallest_string;

    #[test]
    fn example1() {
        assert_eq!(get_smallest_string("zbbz".into(), 3), "aaaz");
    }

    #[test]
    fn example2() {
        assert_eq!(get_smallest_string("xaxcd".into(), 4), "aawcd");
    }

    #[test]
    fn example3() {
        assert_eq!(get_smallest_string("lol".into(), 0), "lol");
    }
}
