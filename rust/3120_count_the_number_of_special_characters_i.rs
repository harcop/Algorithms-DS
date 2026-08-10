/// LeetCode #3120 - Count the Number of Special Characters I
fn number_of_special_chars(word: String) -> i32 {
    let mut s = [false; 128];
    for ch in word.chars() {
        s[ch as usize] = true;
    }
    let mut ans = 0;
    for i in 0..26u8 {
        if s[(b'a' + i) as usize] && s[(b'A' + i) as usize] {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", number_of_special_chars("aaAbcBC".into()));
}

#[cfg(test)]
mod tests {
    use super::number_of_special_chars;

    #[test]
    fn example1() {
        assert_eq!(number_of_special_chars("aaAbcBC".into()), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(number_of_special_chars("abc".into()), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(number_of_special_chars("abBCab".into()), 1);
    }
}
