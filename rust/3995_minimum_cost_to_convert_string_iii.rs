/// LeetCode #3995 - Minimum Cost to Convert String III
fn min_cost(source: String, target: String, rules: Vec<(String, String)>, costs: Vec<i32>) -> i32 {
    let src = source.as_bytes();
    let tgt = target.as_bytes();
    let n = src.len();
    let inf = i32::MAX / 4;
    let mut dp = vec![inf; n + 1];
    dp[0] = 0;
    let prepared: Vec<(&[u8], &[u8], i32)> = rules
        .iter()
        .zip(costs.iter())
        .map(|((pat, rep), &cost)| {
            let stars = pat.bytes().filter(|&c| c == b'*').count() as i32;
            (pat.as_bytes(), rep.as_bytes(), cost + stars)
        })
        .collect();
    for i in 0..n {
        if dp[i] >= inf {
            continue;
        }
        if src[i] == tgt[i] {
            dp[i + 1] = dp[i + 1].min(dp[i]);
        }
        for &(pat, rep, cost) in &prepared {
            let len = pat.len();
            if i + len > n || &tgt[i..i + len] != rep {
                continue;
            }
            let ok = pat
                .iter()
                .zip(&src[i..i + len])
                .all(|(&p, &s)| p == b'*' || p == s);
            if ok {
                dp[i + len] = dp[i + len].min(dp[i] + cost);
            }
        }
    }
    if dp[n] >= inf {
        -1
    } else {
        dp[n]
    }
}

fn main() {
    println!(
        "{}",
        min_cost(
            "hello".into(),
            "world".into(),
            vec![("he".into(), "wo".into()), ("llo".into(), "rld".into())],
            vec![3, 4]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::min_cost;

    #[test]
    fn example1() {
        assert_eq!(
            min_cost(
                "hello".into(),
                "world".into(),
                vec![("he".into(), "wo".into()), ("llo".into(), "rld".into())],
                vec![3, 4]
            ),
            7
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_cost(
                "cat".into(),
                "dog".into(),
                vec![("c*t".into(), "dog".into())],
                vec![2]
            ),
            3
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            min_cost(
                "test".into(),
                "next".into(),
                vec![("*e*t".into(), "next".into())],
                vec![4]
            ),
            6
        );
    }

    #[test]
    fn example4() {
        assert_eq!(
            min_cost(
                "ab".into(),
                "bc".into(),
                vec![("a*".into(), "bd".into())],
                vec![9]
            ),
            -1
        );
    }
}
