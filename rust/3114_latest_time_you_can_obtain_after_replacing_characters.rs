/// LeetCode #3114 - Latest Time You Can Obtain After Replacing Characters
fn find_latest_time(s: String) -> String {
    let mut cs: Vec<u8> = s.into_bytes();
    if cs[0] == b'?' {
        cs[0] = if cs[1] == b'?' || cs[1] < b'2' { b'1' } else { b'0' };
    }
    if cs[1] == b'?' {
        cs[1] = if cs[0] == b'1' { b'1' } else { b'9' };
    }
    if cs[3] == b'?' {
        cs[3] = b'5';
    }
    if cs[4] == b'?' {
        cs[4] = b'9';
    }
    String::from_utf8(cs).unwrap()
}

fn main() {
    println!("{}", find_latest_time("1?:?4".into()));
}

#[cfg(test)]
mod tests {
    use super::find_latest_time;

    #[test]
    fn example1() {
        assert_eq!(find_latest_time("1?:?4".into()), "11:54");
    }

    #[test]
    fn example2() {
        assert_eq!(find_latest_time("0?:5?".into()), "09:59");
    }
}
