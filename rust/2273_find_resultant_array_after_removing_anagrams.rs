/// LeetCode #2273 - Find Resultant Array After Removing Anagrams
fn remove_anagrams(words: Vec<String>) -> Vec<String> {
    let mut ans = Vec::new();
    let mut i = 0;

    while i < words.len() {
        let mut j = i + 1;
        while j < words.len() && is_anagram(&words[i], &words[j]) {
            j += 1;
        }
        ans.push(words[i].clone());
        i = j;
    }

    ans
}

fn is_anagram(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut count = [0i32; 26];
    for c in a.bytes() {
        count[(c - b'a') as usize] += 1;
    }
    for c in b.bytes() {
        count[(c - b'a') as usize] -= 1;
    }
    count.iter().all(|&c| c == 0)
}

fn main() {
    println!(
        "{:?}",
        remove_anagrams(vec![
            "abba".into(),
            "baba".into(),
            "bbaa".into(),
            "cd".into(),
            "cd".into()
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::remove_anagrams;

    #[test]
    fn example_one() {
        assert_eq!(
            remove_anagrams(vec![
                "abba".into(),
                "baba".into(),
                "bbaa".into(),
                "cd".into(),
                "cd".into()
            ]),
            vec!["abba".to_string(), "cd".to_string()]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            remove_anagrams(vec!["a".into(), "b".into(), "c".into(), "d".into(), "e".into()]),
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string()
            ]
        );
    }
}
