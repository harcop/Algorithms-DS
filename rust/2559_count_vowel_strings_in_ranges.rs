/// LeetCode #2559 - Count Vowel Strings in Ranges
fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = words.len();
    let mut prefix = vec![0; n + 1];
    for (i, w) in words.iter().enumerate() {
        let chars: Vec<char> = w.chars().collect();
        let good = is_vowel(chars[0]) && is_vowel(chars[chars.len() - 1]);
        prefix[i + 1] = prefix[i] + if good { 1 } else { 0 };
    }

    queries
        .into_iter()
        .map(|q| {
            let l = q[0] as usize;
            let r = q[1] as usize;
            prefix[r + 1] - prefix[l]
        })
        .collect()
}

fn main() {
    let words = vec![
        "aba".to_string(),
        "bcb".to_string(),
        "ece".to_string(),
        "aa".to_string(),
        "e".to_string(),
    ];
    let queries = vec![vec![0, 2], vec![1, 4], vec![1, 1]];
    println!("{:?}", vowel_strings(words, queries));
}

#[cfg(test)]
mod tests {
    use super::vowel_strings;

    #[test]
    fn example_one() {
        let words = vec![
            "aba".to_string(),
            "bcb".to_string(),
            "ece".to_string(),
            "aa".to_string(),
            "e".to_string(),
        ];
        let queries = vec![vec![0, 2], vec![1, 4], vec![1, 1]];
        assert_eq!(vowel_strings(words, queries), vec![2, 3, 0]);
    }

    #[test]
    fn example_two() {
        let words = vec!["a".to_string(), "e".to_string(), "i".to_string()];
        let queries = vec![vec![0, 2], vec![0, 1], vec![2, 2]];
        assert_eq!(vowel_strings(words, queries), vec![3, 2, 1]);
    }
}
