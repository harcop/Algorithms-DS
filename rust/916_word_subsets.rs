/// LeetCode #916 - Word Subsets
fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
    let mut need = [0i32; 26];
    for w in words2 {
        let mut local = [0i32; 26];
        for b in w.bytes() {
            local[(b - b'a') as usize] += 1;
        }
        for i in 0..26 {
            need[i] = need[i].max(local[i]);
        }
    }
    let mut ans = Vec::new();
    for w in words1 {
        let mut cnt = [0i32; 26];
        for b in w.bytes() {
            cnt[(b - b'a') as usize] += 1;
        }
        let ok = (0..26).all(|i| cnt[i] >= need[i]);
        if ok {
            ans.push(w);
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        word_subsets(
            vec![
                "amazon".into(),
                "apple".into(),
                "facebook".into(),
                "google".into(),
                "leetcode".into(),
            ],
            vec!["e".into(), "o".into()],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::word_subsets;

    #[test]
    fn example_one() {
        let a = vec![
            "amazon".into(),
            "apple".into(),
            "facebook".into(),
            "google".into(),
            "leetcode".into(),
        ];
        let b = vec!["e".into(), "o".into()];
        let mut out = word_subsets(a, b);
        out.sort();
        let mut exp: Vec<String> = vec![
            "facebook".into(),
            "google".into(),
            "leetcode".into(),
        ];
        exp.sort();
        assert_eq!(out, exp);
    }

    #[test]
    fn example_two() {
        let a = vec!["amazon".into(), "apple".into(), "facebook".into(), "google".into(), "leetcode".into()];
        let b = vec!["l".into(), "e".into()];
        let mut out = word_subsets(a, b);
        out.sort();
        let mut exp: Vec<String> = vec!["apple".into(), "google".into(), "leetcode".into()];
        exp.sort();
        assert_eq!(out, exp);
    }
}
