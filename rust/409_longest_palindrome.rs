/// LeetCode #409 - Longest Palindrome
fn longest_palindrome(s: String) -> i32 {
    let mut cnt = [0i32; 128];
    for b in s.bytes() {
        cnt[b as usize] += 1;
    }
    let mut len = 0i32;
    let mut odd = false;
    for &c in &cnt {
        len += (c / 2) * 2;
        if c % 2 == 1 {
            odd = true;
        }
    }
    if odd { len + 1 } else { len }
}

fn main() {
    println!("{}", longest_palindrome("abccccdd".into()));
}

#[cfg(test)]
mod tests {
    use super::longest_palindrome;

    #[test]
    fn example_one() {
        assert_eq!(longest_palindrome("abccccdd".into()), 7);
    }
}
