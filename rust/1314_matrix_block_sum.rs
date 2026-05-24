/// LeetCode #1314 - Matrix Block Sum
fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let m = mat.len();
    if m == 0 {
        return vec![];
    }
    let n = mat[0].len();
    let mut pref = vec![vec![0i64; n + 1]; m + 1];
    for i in 0..m {
        for j in 0..n {
            pref[i + 1][j + 1] = pref[i][j + 1] + pref[i + 1][j] - pref[i][j] + mat[i][j] as i64;
        }
    }
    let k = k as i32;
    let mut ans = vec![vec![0; n]; m];
    for i in 0..m {
        for j in 0..n {
            let r1 = (i as i32 - k).max(0) as usize;
            let c1 = (j as i32 - k).max(0) as usize;
            let r2 = (i as i32 + k).min(m as i32 - 1) as usize;
            let c2 = (j as i32 + k).min(n as i32 - 1) as usize;
            let sum = pref[r2 + 1][c2 + 1] - pref[r1][c2 + 1] - pref[r2 + 1][c1] + pref[r1][c1];
            ans[i][j] = sum as i32;
        }
    }
    ans
}

fn main() {
    println!("{:?}", matrix_block_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1));
}

#[cfg(test)]
mod tests {
    use super::matrix_block_sum;

    #[test]
    fn example_one() {
        assert_eq!(matrix_block_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1), vec![vec![12, 21, 16], vec![27, 45, 33], vec![24, 39, 28]]);
    }
}
