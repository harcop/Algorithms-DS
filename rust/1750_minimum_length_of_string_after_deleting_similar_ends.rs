/// LeetCode #1750 - Minimum Length of String After Deleting Similar Ends
fn minimum_length(s: String) -> i32 {
    let b = s.as_bytes();
    let mut l = 0usize;
    let mut r = b.len();
    while l < r && b[l] == b[r - 1] {
        let c = b[l];
        while l < r && b[l] == c {
            l += 1;
        }
        while l < r && b[r - 1] == c {
            r -= 1;
        }
    }
    (r - l) as i32
}
fn main() { println!("{}", minimum_length("aabccab".into())); }
#[cfg(test)]
mod tests {
    use super::minimum_length;
    #[test]
    fn example_one() { assert_eq!(minimum_length("ca".into()), 2); }
    #[test]
    fn example_two() { assert_eq!(minimum_length("abccba".into()), 0); }
}
