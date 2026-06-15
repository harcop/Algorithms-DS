/// LeetCode #1876 - Substrings of Size Three with Distinct Characters
fn count_good_substrings(s: String) -> i32 {
    let s = s.as_bytes();
    let mut ans = 0i32;
    let mut mask = 0u32;
    let mut l = 0usize;
    for r in 0..s.len() {
        let x = (s[r] - b'a') as u32;
        while mask >> x & 1 == 1 {
            let y = (s[l] - b'a') as u32;
            mask ^= 1 << y;
            l += 1;
        }
        mask |= 1 << x;
        if r - l + 1 >= 3 {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", count_good_substrings("xyzzaz".into()));
}

#[cfg(test)]
mod tests {
    use super::count_good_substrings;

    #[test]
    fn example_one() {
        assert_eq!(count_good_substrings("xyzzaz".into()), 1);
    }
}
