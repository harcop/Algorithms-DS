/// LeetCode #1957 - Delete Characters to Make Fancy String
fn make_fancy_string(s: String) -> String {
    let bytes = s.as_bytes();
    let mut ans = Vec::new();
    for (i, &b) in bytes.iter().enumerate() {
        if i < 2 || b != bytes[i - 1] || b != bytes[i - 2] {
            ans.push(b);
        }
    }
    String::from_utf8(ans).unwrap()
}

fn main() {
    println!("{}", make_fancy_string("leeetcode".into()));
}

#[cfg(test)]
mod tests {
    use super::make_fancy_string;

    #[test]
    fn example_one() {
        assert_eq!(make_fancy_string("leeetcode".into()), "leetcode");
    }

    #[test]
    fn example_two() {
        assert_eq!(make_fancy_string("aaabaaaa".into()), "aabaa");
    }

    #[test]
    fn example_three() {
        assert_eq!(make_fancy_string("aab".into()), "aab");
    }
}
