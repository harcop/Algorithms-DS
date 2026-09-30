/// LeetCode #4016 - Maximum Area of Two Non-Overlapping Square Submatrices
fn max_area(mat: Vec<Vec<i32>>) -> i32 {
    fn calc(mat: &[Vec<i32>]) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut f = vec![vec![0i32; n + 1]; m + 1];
        let mut g = vec![0i32; m + 1];
        let mut suf = vec![0i32; m + 1];
        for i in (1..m).rev() {
            for j in (0..n).rev() {
                if mat[i][j] != 0 {
                    f[i][j] = f[i + 1][j].min(f[i][j + 1]).min(f[i + 1][j + 1]) + 1;
                    g[i] = g[i].max(f[i][j]);
                }
            }
            suf[i] = suf[i + 1].max(g[i]);
        }
        f = vec![vec![0i32; n + 1]; m + 1];
        g = vec![0i32; m + 1];
        let mut pre = vec![0i32; m + 1];
        for i in 1..=m {
            for j in 1..=n {
                if mat[i - 1][j - 1] != 0 {
                    f[i][j] = f[i - 1][j].min(f[i][j - 1]).min(f[i - 1][j - 1]) + 1;
                    g[i] = g[i].max(f[i][j]);
                }
            }
            pre[i] = pre[i - 1].max(g[i]);
        }
        let mut ans = 0i32;
        for i in 1..m {
            let t = pre[i].min(suf[i]);
            ans = ans.max(t * t);
        }
        ans
    }

    fn transpose(mat: &[Vec<i32>]) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let mut ans = vec![vec![0; m]; n];
        for i in 0..m {
            for j in 0..n {
                ans[j][i] = mat[i][j];
            }
        }
        ans
    }

    calc(&mat).max(calc(&transpose(&mat)))
}

fn main() {
    println!(
        "{}",
        max_area(vec![vec![1, 1, 1, 0], vec![1, 1, 1, 1], vec![0, 0, 1, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::max_area;

    #[test]
    fn example1() {
        assert_eq!(
            max_area(vec![vec![1, 1, 1, 0], vec![1, 1, 1, 1], vec![0, 0, 1, 1]]),
            4
        );
    }

    #[test]
    fn example2() {
        assert_eq!(max_area(vec![vec![0, 1], vec![1, 0]]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(max_area(vec![vec![0, 0], vec![0, 1]]), 0);
    }
}
