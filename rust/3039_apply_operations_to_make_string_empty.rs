/// LeetCode #3039 - Apply Operations to Make String Empty
fn last_non_empty_string(s: String) -> String {
    let mut cur = s;
    let mut prev = String::new();
    loop {
        if cur.is_empty() {
            break;
        }
        prev = cur.clone();
        let mut seen = std::collections::HashSet::new();
        cur = cur
            .chars()
            .filter(|&c| {
                if seen.contains(&c) {
                    true
                } else {
                    seen.insert(c);
                    false
                }
            })
            .collect();
    }
    prev
}

fn main() {
    println!("{}", last_non_empty_string("aabcbbca".into()));
}

#[cfg(test)]
mod tests {
    use super::last_non_empty_string;

    #[test]
    fn example1() {
        assert_eq!(last_non_empty_string("aabcbbca".into()), "ba");
    }

    #[test]
    fn example2() {
        assert_eq!(last_non_empty_string("abcd".into()), "abcd");
    }
}
