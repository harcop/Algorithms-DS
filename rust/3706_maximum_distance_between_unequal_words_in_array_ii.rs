/// LeetCode #3706 - Maximum Distance Between Unequal Words in Array II
fn max_distance(words: Vec<String>) -> i32 {
    let n = words.len();
    for i in 0..=n / 2 {
        if words[n - 1 - i] != words[0] || words[i] != words[n - 1] {
            return (n - i) as i32;
        }
    }
    0
}

fn main() {
    println!(
        "{}",
        max_distance(vec![
            "leetcode".into(),
            "leetcode".into(),
            "codeforces".into()
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::max_distance;

    #[test]
    fn example1() {
        assert_eq!(
            max_distance(vec![
                "leetcode".into(),
                "leetcode".into(),
                "codeforces".into()
            ]),
            3
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_distance(vec![
                "a".into(),
                "b".into(),
                "c".into(),
                "a".into(),
                "a".into()
            ]),
            4
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            max_distance(vec!["z".into(), "z".into(), "z".into()]),
            0
        );
    }
}
