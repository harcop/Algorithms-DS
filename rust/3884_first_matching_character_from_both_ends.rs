/// LeetCode #3884 - First Matching Character from Both Ends
fn first_matching(s: String) -> i32 {
    let b = s.as_bytes();
    let n = b.len();
    for i in 0..=n / 2 {
        if b[i] == b[n - 1 - i] {
            return i as i32;
        }
    }
    -1
}

fn main() {
    println!("{}", first_matching("abcacbd".into()));
}

#[cfg(test)]
mod tests {
    use super::first_matching;

    #[test]
    fn example1() {
        assert_eq!(first_matching("abcacbd".into()), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(first_matching("abc".into()), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(first_matching("abcdab".into()), -1);
    }
}
