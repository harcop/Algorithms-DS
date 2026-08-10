/// LeetCode #3121 - Count the Number of Special Characters II
fn number_of_special_chars(word: String) -> i32 {
    let mut first = [0i32; 128];
    let mut last = [0i32; 128];
    for (i, ch) in word.chars().enumerate() {
        let j = ch as usize;
        let pos = (i + 1) as i32;
        if first[j] == 0 {
            first[j] = pos;
        }
        last[j] = pos;
    }
    let mut ans = 0;
    for i in 0..26u8 {
        let a = (b'a' + i) as usize;
        let b = (b'A' + i) as usize;
        if last[a] > 0 && last[a] < first[b] {
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
        assert_eq!(number_of_special_chars("AbBCab".into()), 0);
    }
}
