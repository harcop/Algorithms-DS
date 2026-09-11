/// LeetCode #3696 - Maximum Distance Between Unequal Words in Array I (premium)
fn max_distance(words: Vec<String>) -> i32 {
    let n = words.len();
    let mut ans = 0i32;
    for i in 0..n {
        if words[i] != words[0] {
            ans = ans.max((i + 1) as i32);
        }
        if words[i] != words[n - 1] {
            ans = ans.max((n - i) as i32);
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        max_distance(vec![
            "leetcode".to_string(),
            "leetcode".to_string(),
            "codeforces".to_string()
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
                "leetcode".to_string(),
                "leetcode".to_string(),
                "codeforces".to_string()
            ]),
            3
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_distance(vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "a".to_string(),
                "a".to_string()
            ]),
            4
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            max_distance(vec!["z".to_string(), "z".to_string(), "z".to_string()]),
            0
        );
    }
}
