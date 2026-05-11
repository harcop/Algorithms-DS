/// LeetCode #647 - Palindromic Substrings
fn count_substrings(s: String) -> i32 {
    let b = s.as_bytes();
    let n = b.len();
    let mut ans = 0i32;
    for c in 0..2 * n {
        let mut l = c / 2;
        let mut r = l + c % 2;
        while r < n && l <= r && b[l] == b[r] {
            ans += 1;
            if l == 0 { break; }
            l -= 1;
            r += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", count_substrings("abc".into()));
}

#[cfg(test)]
mod tests {
    use super::count_substrings;

    #[test]
    fn example_one() {
        assert_eq!(count_substrings("abc".into()), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_substrings("aaa".into()), 6);
    }
}
