/// LeetCode #2947 - Count Beautiful Substrings I
fn beautiful_substrings(s: String, k: i32) -> i32 {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let vowels = b"aeiou";
    let mut ans = 0;
    for i in 0..n {
        let mut v = 0;
        for j in i..n {
            if vowels.contains(&bytes[j]) {
                v += 1;
            }
            let c = (j - i + 1) as i32 - v;
            if v == c && v * c % k == 0 {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", beautiful_substrings("baeyh".into(), 2));
}

#[cfg(test)]
mod tests {
    use super::beautiful_substrings;

    #[test]
    fn example_one() {
        assert_eq!(beautiful_substrings("baeyh".into(), 2), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(beautiful_substrings("abba".into(), 1), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(beautiful_substrings("bcdf".into(), 1), 0);
    }
}
