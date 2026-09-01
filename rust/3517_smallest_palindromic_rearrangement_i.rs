/// LeetCode #3517 - Smallest Palindromic Rearrangement I
fn smallest_palindrome(s: String) -> String {
    let mut cnt = [0usize; 26];
    for b in s.bytes() {
        cnt[(b - b'a') as usize] += 1;
    }
    let mut left = String::new();
    let mut mid = String::new();
    for (i, &c) in cnt.iter().enumerate() {
        let ch = (b'a' + i as u8) as char;
        left.extend(std::iter::repeat(ch).take(c / 2));
        if c % 2 == 1 {
            mid.push(ch);
        }
    }
    let right: String = left.chars().rev().collect();
    left + &mid + &right
}

fn main() {
    println!("{}", smallest_palindrome("babab".into()));
}

#[cfg(test)]
mod tests {
    use super::smallest_palindrome;

    #[test]
    fn example1() {
        assert_eq!(smallest_palindrome("z".into()), "z");
    }

    #[test]
    fn example2() {
        assert_eq!(smallest_palindrome("babab".into()), "abbba");
    }

    #[test]
    fn example3() {
        assert_eq!(smallest_palindrome("daccad".into()), "acddca");
    }
}
