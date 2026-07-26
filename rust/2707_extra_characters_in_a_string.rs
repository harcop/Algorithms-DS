/// LeetCode #2707 - Extra Characters in a String
use std::collections::HashSet;

fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
    let ss: HashSet<String> = dictionary.into_iter().collect();
    let n = s.len();
    let mut f = vec![0; n + 1];
    for i in 1..=n {
        f[i] = f[i - 1] + 1;
        for j in 0..i {
            if ss.contains(&s[j..i]) {
                f[i] = f[i].min(f[j]);
            }
        }
    }
    f[n]
}

fn main() {
    println!(
        "{}",
        min_extra_char(
            "leetscode".into(),
            vec!["leet".into(), "code".into(), "leetcode".into()]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::min_extra_char;

    #[test]
    fn example_one() {
        assert_eq!(
            min_extra_char(
                "leetscode".into(),
                vec!["leet".into(), "code".into(), "leetcode".into()]
            ),
            1
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            min_extra_char("sayhelloworld".into(), vec!["hello".into(), "world".into()]),
            3
        );
    }
}
