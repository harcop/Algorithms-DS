/// LeetCode #3853 - Merge Close Characters
use std::collections::HashMap;

fn merge_characters(s: String, k: i32) -> String {
    let mut last: HashMap<u8, i32> = HashMap::new();
    let mut ans = Vec::new();
    for c in s.bytes() {
        let cur = ans.len() as i32;
        if last.get(&c).map(|&i| cur - i <= k).unwrap_or(false) {
            continue;
        }
        ans.push(c);
        last.insert(c, cur);
    }
    String::from_utf8(ans).unwrap()
}

fn main() {
    println!("{}", merge_characters("abca".into(), 3));
}

#[cfg(test)]
mod tests {
    use super::merge_characters;

    #[test]
    fn example1() {
        assert_eq!(merge_characters("abca".into(), 3), "abc");
    }

    #[test]
    fn example2() {
        assert_eq!(merge_characters("aabca".into(), 2), "abca");
    }

    #[test]
    fn example3() {
        assert_eq!(merge_characters("yybyzybz".into(), 2), "ybzybz");
    }
}
