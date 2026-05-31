/// LeetCode #1605 - Find Valid Matrix Given Row And Column Sums
fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
    let n = row_sum.len();
    let m = col_sum.len();
    let mut ans = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            ans[i][j] = row_sum[i].min(col_sum[j]);
            row_sum[i] -= ans[i][j];
            col_sum[j] -= ans[i][j];
        }
    }
    ans
}
fn main() { println!("{:?}", restore_matrix(vec![3,8], vec![4,7])); }
#[cfg(test)]
mod tests {
    use super::restore_matrix;
    #[test]
    fn example_one() { assert_eq!(restore_matrix(vec![3,8], vec![4,7]), vec![vec![3,0],vec![1,7]]); }
}