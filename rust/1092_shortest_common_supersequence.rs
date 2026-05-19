/// LeetCode #1092 - Shortest Common Supersequence
fn shortest_common_supersequence(str1: String, str2: String) -> String {
    let a: Vec<u8> = str1.bytes().collect();
    let b: Vec<u8> = str2.bytes().collect();
    let n = a.len();
    let m = b.len();
    let mut dp = vec![vec![0usize; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }
    let mut i = n;
    let mut j = m;
    let mut out = Vec::new();
    while i > 0 && j > 0 {
        if a[i - 1] == b[j - 1] {
            out.push(a[i - 1]);
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] >= dp[i][j - 1] {
            out.push(a[i - 1]);
            i -= 1;
        } else {
            out.push(b[j - 1]);
            j -= 1;
        }
    }
    while i > 0 {
        out.push(a[i - 1]);
        i -= 1;
    }
    while j > 0 {
        out.push(b[j - 1]);
        j -= 1;
    }
    out.reverse();
    String::from_utf8(out).unwrap()
}

fn main() {
    println!(
        "{}",
        shortest_common_supersequence("abac".into(), "cab".into())
    );
}

#[cfg(test)]
mod tests {
    use super::shortest_common_supersequence;

    #[test]
    fn example_one() {
        assert_eq!(
            shortest_common_supersequence("abac".into(), "cab".into()),
            "cabac"
        );
    }
}
