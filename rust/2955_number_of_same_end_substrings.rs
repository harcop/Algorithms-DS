/// LeetCode #2955 - Number of Same-End Substrings (Premium)
fn same_end_substring_count(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut prefix = vec![[0i32; 26]; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i];
        prefix[i + 1][(bytes[i] - b'a') as usize] += 1;
    }
    queries
        .into_iter()
        .map(|q| {
            let l = q[0] as usize;
            let r = q[1] as usize;
            let mut t = (r - l + 1) as i32;
            for c in 0..26 {
                let x = prefix[r + 1][c] - prefix[l][c];
                t += x * (x - 1) / 2;
            }
            t
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        same_end_substring_count(
            "abcaab".into(),
            vec![vec![0, 0], vec![1, 4], vec![2, 5], vec![0, 5]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::same_end_substring_count;

    #[test]
    fn example_one() {
        assert_eq!(
            same_end_substring_count(
                "abcaab".into(),
                vec![vec![0, 0], vec![1, 4], vec![2, 5], vec![0, 5]]
            ),
            vec![1, 5, 5, 10]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            same_end_substring_count("abcd".into(), vec![vec![0, 3]]),
            vec![4]
        );
    }
}
