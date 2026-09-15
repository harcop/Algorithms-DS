/// LeetCode #3760 - Maximum Substrings With Distinct Start
fn max_distinct(s: String) -> i32 {
    let mut seen = [false; 26];
    let mut ans = 0;
    for c in s.bytes() {
        let i = (c - b'a') as usize;
        if !seen[i] {
            seen[i] = true;
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", max_distinct("abab".into()));
}

#[cfg(test)]
mod tests {
    use super::max_distinct;

    #[test]
    fn example1() {
        assert_eq!(max_distinct("abab".into()), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(max_distinct("abcd".into()), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(max_distinct("aaaa".into()), 1);
    }
}
