/// LeetCode #4019 - Merge Close Characters II
fn merge_characters(s: String, k: i32) -> String {
    let k = k as usize;
    let mut last = [usize::MAX; 26];
    let mut ans = Vec::new();
    for c in s.bytes() {
        let cur = ans.len();
        let idx = (c - b'a') as usize;
        if last[idx] != usize::MAX && cur - last[idx] <= k {
            continue;
        }
        ans.push(c);
        last[idx] = cur;
    }
    String::from_utf8(ans).unwrap()
}

fn main() {
    println!("{}", merge_characters("abca".to_string(), 3));
}

#[cfg(test)]
mod tests {
    use super::merge_characters;

    #[test]
    fn example1() {
        assert_eq!(merge_characters("abca".to_string(), 3), "abc");
    }

    #[test]
    fn example2() {
        assert_eq!(merge_characters("aabca".to_string(), 2), "abca");
    }

    #[test]
    fn example3() {
        assert_eq!(merge_characters("yybyzybz".to_string(), 2), "ybzybz");
    }
}
