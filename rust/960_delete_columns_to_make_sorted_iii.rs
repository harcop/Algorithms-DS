/// LeetCode #960 - Delete Columns to Make Sorted III

fn min_deletion_size(strs: Vec<String>) -> i32 {
    if strs.is_empty() {
        return 0;
    }
    let rows = strs.len();
    let cols = strs[0].len();
    let mut dp = vec![1i32; cols];
    for j in 1..cols {
        for i in 0..j {
            let ok = (0..rows).all(|r| strs[r].as_bytes()[i] <= strs[r].as_bytes()[j]);
            if ok {
                dp[j] = dp[j].max(dp[i] + 1);
            }
        }
    }
    (cols as i32) - *dp.iter().max().unwrap_or(&0)
}

fn main() {
    println!("{}", min_deletion_size(vec!["babca".into(), "bbazb".into()]));
}

#[cfg(test)]
mod tests {
    use super::min_deletion_size;

    #[test]
    fn example_one() {
        assert_eq!(min_deletion_size(vec!["babca".into(), "bbazb".into()]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_deletion_size(vec!["edcba".into()]), 4);
    }

    #[test]
    fn example_three() {
        assert_eq!(
            min_deletion_size(vec!["ghi".into(), "def".into(), "abc".into()]),
            0
        );
    }
}
