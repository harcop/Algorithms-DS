/// LeetCode #2901 - Longest Unequal Adjacent Groups Subsequence II
fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
    fn hamming_one(a: &str, b: &str) -> bool {
        if a.len() != b.len() {
            return false;
        }
        a.bytes().zip(b.bytes()).filter(|(x, y)| x != y).count() == 1
    }

    let n = words.len();
    let mut f = vec![1; n];
    let mut prev = vec![usize::MAX; n];
    let mut best = 0;

    for i in 0..n {
        for j in 0..i {
            if groups[i] != groups[j]
                && f[i] < f[j] + 1
                && hamming_one(&words[i], &words[j])
            {
                f[i] = f[j] + 1;
                prev[i] = j;
            }
        }
        if f[i] > f[best] {
            best = i;
        }
    }

    let mut ans = Vec::new();
    let mut cur = best;
    loop {
        ans.push(words[cur].clone());
        if prev[cur] == usize::MAX {
            break;
        }
        cur = prev[cur];
    }
    ans.reverse();
    ans
}

fn main() {
    let words = vec!["bab".into(), "dab".into(), "cab".into()];
    println!("{:?}", get_words_in_longest_subsequence(words, vec![1, 2, 2]));
}

#[cfg(test)]
mod tests {
    use super::get_words_in_longest_subsequence;

    #[test]
    fn example_one() {
        let ans = get_words_in_longest_subsequence(
            vec!["bab".into(), "dab".into(), "cab".into()],
            vec![1, 2, 2],
        );
        assert!(
            ans == vec!["bab".to_string(), "cab".to_string()]
                || ans == vec!["bab".to_string(), "dab".to_string()]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            get_words_in_longest_subsequence(
                vec!["a".into(), "b".into(), "c".into(), "d".into()],
                vec![1, 2, 3, 4]
            ),
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string()
            ]
        );
    }
}
