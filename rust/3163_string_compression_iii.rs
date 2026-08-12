/// LeetCode #3163 - String Compression III
fn compressed_string(word: String) -> String {
    let bytes = word.as_bytes();
    let mut ans = String::new();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        let mut j = i;
        while j < bytes.len() && bytes[j] == c && j - i < 9 {
            j += 1;
        }
        ans.push(char::from_digit((j - i) as u32, 10).unwrap());
        ans.push(c as char);
        i = j;
    }
    ans
}

fn main() {
    println!("{}", compressed_string("abcde".into()));
}

#[cfg(test)]
mod tests {
    use super::compressed_string;

    #[test]
    fn example1() {
        assert_eq!(compressed_string("abcde".into()), "1a1b1c1d1e");
    }

    #[test]
    fn example2() {
        assert_eq!(
            compressed_string("aaaaaaaaaaaaaabb".into()),
            "9a5a2b"
        );
    }
}
