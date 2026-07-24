/// LeetCode #2645 - Minimum Additions to Make Valid String
fn add_minimum(word: String) -> i32 {
    let s = b"abc";
    let bytes = word.as_bytes();
    let n = bytes.len();
    let mut ans = 0;
    let mut i = 0usize;
    let mut j = 0usize;
    while j < n {
        if bytes[j] != s[i] {
            ans += 1;
        } else {
            j += 1;
        }
        i = (i + 1) % 3;
    }
    if bytes[n - 1] == b'b' {
        ans += 1;
    } else if bytes[n - 1] == b'a' {
        ans += 2;
    }
    ans
}

fn main() {
    println!("{}", add_minimum("b".into()));
}

#[cfg(test)]
mod tests {
    use super::add_minimum;

    #[test]
    fn example_one() {
        assert_eq!(add_minimum("b".into()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(add_minimum("aaa".into()), 6);
    }

    #[test]
    fn example_three() {
        assert_eq!(add_minimum("abc".into()), 0);
    }
}
