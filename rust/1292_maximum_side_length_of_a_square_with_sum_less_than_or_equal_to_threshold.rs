/// LeetCode #1292 - Maximum Side Length of a Square with Sum Less than or Equal to Threshold
fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
    let m = mat.len();
    if m == 0 {
        return 0;
    }
    let n = mat[0].len();
    let mut pre = vec![vec![0i64; n + 1]; m + 1];
    for i in 0..m {
        for j in 0..n {
            pre[i + 1][j + 1] =
                pre[i][j + 1] + pre[i + 1][j] - pre[i][j] + mat[i][j] as i64;
        }
    }
    let mut lo = 0i32;
    let mut hi = (m.min(n)) as i32;
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        if has_square(&pre, m, n, mid as usize, threshold as i64) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo
}

fn has_square(pre: &[Vec<i64>], m: usize, n: usize, side: usize, threshold: i64) -> bool {
    if side == 0 {
        return true;
    }
    for i in side..=m {
        for j in side..=n {
            let sum = pre[i][j] - pre[i - side][j] - pre[i][j - side] + pre[i - side][j - side];
            if sum <= threshold {
                return true;
            }
        }
    }
    false
}

fn main() {
    let mat = vec![
        vec![1, 1, 3, 2, 2],
        vec![1, 1, 3, 2, 2],
        vec![1, 1, 3, 2, 2],
        vec![1, 1, 3, 2, 2],
        vec![1, 1, 3, 2, 2],
    ];
    println!("{}", max_side_length(mat, 4));
}

#[cfg(test)]
mod tests {
    use super::max_side_length;

    #[test]
    fn example_one() {
        let mat = vec![
            vec![1, 1, 3, 2, 2],
            vec![1, 1, 3, 2, 2],
            vec![1, 1, 3, 2, 2],
            vec![1, 1, 3, 2, 2],
            vec![1, 1, 3, 2, 2],
        ];
        assert_eq!(max_side_length(mat, 4), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_side_length(vec![vec![2, 2, 2, 2, 2]], 1), 0);
    }
}
