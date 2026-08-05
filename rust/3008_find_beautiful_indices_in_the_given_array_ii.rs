/// LeetCode #3008 - Find Beautiful Indices in the Given Array II
fn kmp_find(text: &[u8], pat: &[u8]) -> Vec<usize> {
    if pat.is_empty() {
        return (0..=text.len()).collect();
    }
    let mut lps = vec![0usize; pat.len()];
    let mut len = 0usize;
    for i in 1..pat.len() {
        while len > 0 && pat[i] != pat[len] {
            len = lps[len - 1];
        }
        if pat[i] == pat[len] {
            len += 1;
        }
        lps[i] = len;
    }

    let mut ans = Vec::new();
    let mut j = 0usize;
    for (i, &c) in text.iter().enumerate() {
        while j > 0 && c != pat[j] {
            j = lps[j - 1];
        }
        if c == pat[j] {
            j += 1;
        }
        if j == pat.len() {
            ans.push(i + 1 - pat.len());
            j = lps[j - 1];
        }
    }
    ans
}

fn beautiful_indices(s: String, a: String, b: String, k: i32) -> Vec<i32> {
    let s = s.as_bytes();
    let a = a.as_bytes();
    let b = b.as_bytes();
    let k = k as i64;

    let pos_a: Vec<i64> = kmp_find(s, a).into_iter().map(|x| x as i64).collect();
    let pos_b: Vec<i64> = kmp_find(s, b).into_iter().map(|x| x as i64).collect();

    let mut ans = Vec::new();
    let mut j = 0usize;
    for &i in &pos_a {
        while j < pos_b.len() && pos_b[j] < i - k {
            j += 1;
        }
        if j < pos_b.len() && (pos_b[j] - i).abs() <= k {
            ans.push(i as i32);
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        beautiful_indices(
            "isawsquirrelnearmysquirrelhouseohmy".into(),
            "my".into(),
            "squirrel".into(),
            15
        )
    );
    println!(
        "{:?}",
        beautiful_indices("abcd".into(), "a".into(), "a".into(), 4)
    );
}

#[cfg(test)]
mod tests {
    use super::beautiful_indices;

    #[test]
    fn example_one() {
        assert_eq!(
            beautiful_indices(
                "isawsquirrelnearmysquirrelhouseohmy".into(),
                "my".into(),
                "squirrel".into(),
                15
            ),
            vec![16, 33]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            beautiful_indices("abcd".into(), "a".into(), "a".into(), 4),
            vec![0]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            beautiful_indices("abcd".into(), "a".into(), "a".into(), 4),
            vec![0]
        );
    }
}
