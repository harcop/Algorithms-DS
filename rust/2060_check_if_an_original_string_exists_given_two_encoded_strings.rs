/// LeetCode #2060 - Check if an Original String Exists Given Two Encoded Strings
use std::collections::HashSet;

fn possibly_equals(s1: String, s2: String) -> bool {
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    let m = s1.len();
    let n = s2.len();
    let mut dp = vec![vec![HashSet::new(); n + 1]; m + 1];
    dp[0][0].insert(0i32);

    for i in 0..=m {
        for j in 0..=n {
            let deltas: Vec<i32> = dp[i][j].iter().copied().collect();
            for &delta in &deltas {
                let mut num = 0i32;
                for p in i..m {
                    if !s1[p].is_ascii_digit() {
                        break;
                    }
                    num = num * 10 + (s1[p] - b'0') as i32;
                    dp[p + 1][j].insert(delta + num);
                }
                num = 0;
                for p in j..n {
                    if !s2[p].is_ascii_digit() {
                        break;
                    }
                    num = num * 10 + (s2[p] - b'0') as i32;
                    dp[i][p + 1].insert(delta - num);
                }
                if i < m && delta < 0 && s1[i].is_ascii_alphabetic() {
                    dp[i + 1][j].insert(delta + 1);
                }
                if j < n && delta > 0 && s2[j].is_ascii_alphabetic() {
                    dp[i][j + 1].insert(delta - 1);
                }
                if i < m && j < n && delta == 0 && s1[i] == s2[j] {
                    dp[i + 1][j + 1].insert(0);
                }
            }
        }
    }
    dp[m][n].contains(&0)
}

fn main() {
    println!(
        "{}",
        possibly_equals("internationalization".into(), "i18n".into())
    );
}

#[cfg(test)]
mod tests {
    use super::possibly_equals;

    #[test]
    fn example_one() {
        assert!(possibly_equals(
            "internationalization".into(),
            "i18n".into()
        ));
    }

    #[test]
    fn example_two() {
        assert!(possibly_equals("l123e".into(), "44".into()));
    }

    #[test]
    fn example_three() {
        assert!(!possibly_equals("a5b".into(), "c5b".into()));
    }

    #[test]
    fn example_four() {
        assert!(possibly_equals("112s".into(), "g841".into()));
    }
}
