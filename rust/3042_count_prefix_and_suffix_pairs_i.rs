/// LeetCode #3042 - Count Prefix and Suffix Pairs I
fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
    let n = words.len();
    let mut ans = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let a = &words[i];
            let b = &words[j];
            if b.len() >= a.len() && b.starts_with(a.as_str()) && b.ends_with(a.as_str()) {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    let words = vec!["a".into(), "aba".into(), "ababa".into(), "aa".into()];
    println!("{}", count_prefix_suffix_pairs(words));
}

#[cfg(test)]
mod tests {
    use super::count_prefix_suffix_pairs;

    #[test]
    fn example1() {
        assert_eq!(
            count_prefix_suffix_pairs(vec![
                "a".into(),
                "aba".into(),
                "ababa".into(),
                "aa".into()
            ]),
            4
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            count_prefix_suffix_pairs(vec!["pa".into(), "papa".into(), "ma".into(), "mama".into()]),
            2
        );
    }

    #[test]
    fn example3() {
        assert_eq!(count_prefix_suffix_pairs(vec!["abab".into(), "ab".into()]), 0);
    }
}
