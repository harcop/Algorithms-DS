/// LeetCode #3485 - Longest Common Prefix of K Strings After Removal
fn lcp_len(a: &str, b: &str) -> i32 {
    a.bytes()
        .zip(b.bytes())
        .take_while(|(x, y)| x == y)
        .count() as i32
}

fn window_lcps(words: &[String], idxs: &[usize], k: usize) -> Vec<i32> {
    let n = words.len();
    let mut lcp = vec![0; n];
    if k == 0 || k > n {
        return lcp;
    }
    for i in 0..=n - k {
        lcp[i] = lcp_len(&words[idxs[i]], &words[idxs[i + k - 1]]);
    }
    lcp
}

fn longest_common_prefix(words: Vec<String>, k: i32) -> Vec<i32> {
    let n = words.len();
    let k = k as usize;
    let mut idxs: Vec<usize> = (0..n).collect();
    idxs.sort_by(|&a, &b| words[a].cmp(&words[b]));
    let lcp = window_lcps(&words, &idxs, k);
    let mut prefix = vec![0; n];
    prefix[0] = lcp[0];
    for i in 0..n - 1 {
        prefix[i + 1] = prefix[i].max(lcp[i + 1]);
    }
    let mut suffix = vec![0; n];
    suffix[n - 1] = lcp[n - 1];
    for i in (0..n - 1).rev() {
        suffix[i] = suffix[i + 1].max(lcp[i]);
    }
    let mx = window_lcps(&words, &idxs, k + 1)
        .into_iter()
        .max()
        .unwrap_or(0);
    let mut result = vec![0; n];
    for i in 0..n {
        let mx1 = if i >= k { prefix[i - k] } else { 0 };
        let mx2 = if i + 1 < n { suffix[i + 1] } else { 0 };
        result[idxs[i]] = mx.max(mx1).max(mx2);
    }
    result
}

fn main() {
    println!(
        "{:?}",
        longest_common_prefix(
            vec![
                "jump".into(),
                "run".into(),
                "run".into(),
                "jump".into(),
                "run".into()
            ],
            2
        )
    );
}

#[cfg(test)]
mod tests {
    use super::longest_common_prefix;

    #[test]
    fn example1() {
        assert_eq!(
            longest_common_prefix(
                vec![
                    "jump".into(),
                    "run".into(),
                    "run".into(),
                    "jump".into(),
                    "run".into()
                ],
                2
            ),
            vec![3, 4, 4, 3, 4]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            longest_common_prefix(vec!["dog".into(), "racer".into(), "car".into()], 2),
            vec![0, 0, 0]
        );
    }
}
