/// LeetCode #3253 - Construct String with Minimum Cost (Easy)
fn minimum_cost(target: String, words: Vec<String>, costs: Vec<i32>) -> i32 {
    const INF: i32 = i32::MAX / 4;
    let t = target.as_bytes();
    let n = t.len();
    let mut dp = vec![INF; n + 1];
    dp[0] = 0;
    for i in 0..n {
        if dp[i] == INF {
            continue;
        }
        for (w, &cost) in words.iter().zip(costs.iter()) {
            let wb = w.as_bytes();
            if i + wb.len() <= n && &t[i..i + wb.len()] == wb {
                dp[i + wb.len()] = dp[i + wb.len()].min(dp[i] + cost);
            }
        }
    }
    if dp[n] >= INF {
        -1
    } else {
        dp[n]
    }
}

fn main() {
    println!(
        "{}",
        minimum_cost(
            "abcdef".into(),
            vec![
                "abdef".into(),
                "abc".into(),
                "d".into(),
                "def".into(),
                "ef".into()
            ],
            vec![100, 1, 1, 10, 5]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_cost;

    #[test]
    fn example1() {
        assert_eq!(
            minimum_cost(
                "abcdef".into(),
                vec![
                    "abdef".into(),
                    "abc".into(),
                    "d".into(),
                    "def".into(),
                    "ef".into()
                ],
                vec![100, 1, 1, 10, 5]
            ),
            7
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            minimum_cost(
                "aaaa".into(),
                vec!["z".into(), "zz".into(), "zzz".into()],
                vec![1, 10, 100]
            ),
            -1
        );
    }
}
