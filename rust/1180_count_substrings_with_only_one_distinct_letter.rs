/// LeetCode #1180 - Count Substrings with Only One Distinct Letter
fn count_letters(s: String) -> i32 {
    let b = s.as_bytes();
    let mut ans = 0i32;
    let mut i = 0usize;
    while i < b.len() {
        let mut j = i;
        while j < b.len() && b[j] == b[i] {
            j += 1;
        }
        let len = (j - i) as i32;
        ans += len * (len + 1) / 2;
        i = j;
    }
    ans
}

fn main() {
    println!("{}", count_letters("aaaba".into()));
}

#[cfg(test)]
mod tests {
    use super::count_letters;

    #[test]
    fn example_one() {
        assert_eq!(count_letters("aaaba".into()), 8);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_letters("aaaaaaaaaa".into()), 55);
    }
}
