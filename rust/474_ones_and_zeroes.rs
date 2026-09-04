/// LeetCode #474 - Ones and Zeroes
fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let mut dp = vec![vec![0; n + 1]; m + 1];
    for s in strs {
        let zeros = s.bytes().filter(|&c| c == b'0').count();
        let ones = s.len() - zeros;
        if zeros > m || ones > n {
            continue;
        }
        for i in (zeros..=m).rev() {
            for j in (ones..=n).rev() {
                dp[i][j] = dp[i][j].max(dp[i - zeros][j - ones] + 1);
            }
        }
    }
    dp[m][n]
}

fn main() {
    println!(
        "{}",
        find_max_form(
            vec!["10".into(), "0001".into(), "111001".into(), "1".into(), "0".into()],
            5,
            3
        )
    );
}

#[cfg(test)]
mod tests {
    use super::find_max_form;

    #[test]
    fn example_one() {
        assert_eq!(
            find_max_form(
                vec![
                    "10".into(),
                    "0001".into(),
                    "111001".into(),
                    "1".into(),
                    "0".into()
                ],
                5,
                3
            ),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_max_form(vec!["10".into(), "0".into(), "1".into()], 1, 1),
            2
        );
    }
}
