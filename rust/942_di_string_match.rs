/// LeetCode #942 - DI String Match

fn di_string_match(s: String) -> Vec<i32> {
    let mut lo = 0i32;
    let mut hi = s.len() as i32;
    let mut ans = Vec::with_capacity(s.len() + 1);
    for c in s.bytes() {
        if c == b'I' {
            ans.push(lo);
            lo += 1;
        } else {
            ans.push(hi);
            hi -= 1;
        }
    }
    ans.push(lo);
    ans
}

fn main() {
    println!("{:?}", di_string_match("IDID".into()));
}

#[cfg(test)]
mod tests {
    use super::di_string_match;

    #[test]
    fn example_one() {
        assert_eq!(di_string_match("IDID".into()), vec![0, 4, 1, 3, 2]);
    }

    #[test]
    fn example_two() {
        assert_eq!(di_string_match("III".into()), vec![0, 1, 2, 3]);
    }
}
