/// LeetCode #2900 - Longest Unequal Adjacent Groups Subsequence I
fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
    let mut ans = Vec::new();
    let mut prev = -1;
    for (word, group) in words.into_iter().zip(groups) {
        if group != prev {
            prev = group;
            ans.push(word);
        }
    }
    ans
}

fn main() {
    let words = vec!["e".into(), "a".into(), "b".into()];
    println!("{:?}", get_longest_subsequence(words, vec![0, 0, 1]));
}

#[cfg(test)]
mod tests {
    use super::get_longest_subsequence;

    #[test]
    fn example_one() {
        assert_eq!(
            get_longest_subsequence(
                vec!["e".into(), "a".into(), "b".into()],
                vec![0, 0, 1]
            ),
            vec!["e".to_string(), "b".to_string()]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            get_longest_subsequence(
                vec!["a".into(), "b".into(), "c".into(), "d".into()],
                vec![1, 0, 1, 1]
            ),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }
}
