/// LeetCode #1632 - Rank Transform Of A Matrix
fn matrix_rank_transform(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = matrix.len();
    let m = matrix[0].len();
    let mut cells = vec![];
    for i in 0..n {
        for j in 0..m {
            cells.push((matrix[i][j], i, j));
        }
    }
    cells.sort_unstable();
    let mut ans = vec![vec![0; m]; n];
    let mut p = 0usize;
    while p < cells.len() {
        let mut q = p;
        while q < cells.len() && cells[q].0 == cells[p].0 { q += 1; }
        let mut row_h = vec![0i32; n];
        let mut col_h = vec![0i32; m];
        for t in p..q {
            let (_, i, j) = cells[t];
            let mut r = 0i32;
            for jj in 0..m { r = r.max(ans[i][jj]); }
            let mut c = 0i32;
            for ii in 0..n { c = c.max(ans[ii][j]); }
            row_h[i] = row_h[i].max(r);
            col_h[j] = col_h[j].max(c);
        }
        for t in p..q {
            let (_, i, j) = cells[t];
            ans[i][j] = row_h[i].max(col_h[j]) + 1;
        }
        p = q;
    }
    ans
}
fn main() { println!("{:?}", matrix_rank_transform(vec![vec![1,2],vec![3,4]])); }
#[cfg(test)]
mod tests {
    use super::matrix_rank_transform;
    #[test]
    fn example_one() { assert_eq!(matrix_rank_transform(vec![vec![1,2],vec![3,4]]), vec![vec![1,2],vec![2,3]]); }
    #[test]
    fn example_two() { assert_eq!(matrix_rank_transform(vec![vec![7,7],vec![7,7]]), vec![vec![1,1],vec![1,1]]); }
}