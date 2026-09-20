/// LeetCode #3836 - Maximum Score Using Exactly K Pairs
fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    let n = nums1.len();
    let m = nums2.len();
    let k = k as usize;
    let neg = i64::MIN / 4;
    let mut f = vec![vec![vec![neg; k + 1]; m + 1]; n + 1];
    f[0][0][0] = 0;
    for i in 0..=n {
        for j in 0..=m {
            for t in 0..=k {
                if i > 0 {
                    f[i][j][t] = f[i][j][t].max(f[i - 1][j][t]);
                }
                if j > 0 {
                    f[i][j][t] = f[i][j][t].max(f[i][j - 1][t]);
                }
                if i > 0 && j > 0 && t > 0 {
                    f[i][j][t] = f[i][j][t].max(
                        f[i - 1][j - 1][t - 1] + nums1[i - 1] as i64 * nums2[j - 1] as i64,
                    );
                }
            }
        }
    }
    f[n][m][k]
}

fn main() {
    println!("{}", max_score(vec![1, 3, 2], vec![4, 5, 1], 2));
}

#[cfg(test)]
mod tests {
    use super::max_score;

    #[test]
    fn example1() {
        assert_eq!(max_score(vec![1, 3, 2], vec![4, 5, 1], 2), 22);
    }

    #[test]
    fn example2() {
        assert_eq!(max_score(vec![-2, 0, 5], vec![-3, 4, -1, 2], 2), 26);
    }

    #[test]
    fn example3() {
        assert_eq!(max_score(vec![-3, -2], vec![1, 2], 2), -7);
    }
}
