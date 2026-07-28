/// LeetCode #2746 - Decremental String Concatenation
fn minimize_concatenated_length(words: Vec<String>) -> i32 {
    let n = words.len();
    let mut f = vec![vec![vec![None; 26]; 26]; n];
    words[0].len() as i32
        + dfs(1, words[0].as_bytes()[0] - b'a', words[0].as_bytes().last().copied().unwrap() - b'a', &words, &mut f)
}

fn dfs(
    i: usize,
    a: u8,
    b: u8,
    words: &[String],
    f: &mut Vec<Vec<Vec<Option<i32>>>>,
) -> i32 {
    if i >= words.len() {
        return 0;
    }
    if let Some(v) = f[i][a as usize][b as usize] {
        return v;
    }
    let s = words[i].as_bytes();
    let m = s.len();
    let next_first = s[0] - b'a';
    let next_last = s[m - 1] - b'a';
    let x = dfs(i + 1, a, next_last, words, f) - if s[0] - b'a' == b { 1 } else { 0 };
    let y = dfs(i + 1, next_first, b, words, f) - if s[m - 1] - b'a' == a { 1 } else { 0 };
    let ans = m as i32 + x.min(y);
    f[i][a as usize][b as usize] = Some(ans);
    ans
}

fn main() {
    println!(
        "{}",
        minimize_concatenated_length(vec!["aa".into(), "ab".into(), "bc".into()])
    );
}

#[cfg(test)]
mod tests {
    use super::minimize_concatenated_length;

    #[test]
    fn example_one() {
        assert_eq!(
            minimize_concatenated_length(vec!["aa".into(), "ab".into(), "bc".into()]),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(minimize_concatenated_length(vec!["ab".into(), "b".into()]), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(
            minimize_concatenated_length(vec!["aaa".into(), "c".into(), "aba".into()]),
            6
        );
    }
}
