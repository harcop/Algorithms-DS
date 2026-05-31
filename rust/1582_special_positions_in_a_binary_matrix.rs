/// LeetCode #1582 - Special Positions In A Binary Matrix
fn num_special(mat: Vec<Vec<i32>>) -> i32 {
    let n = mat.len();
    let m = mat[0].len();
    let row: Vec<i32> = mat.iter().map(|r| r.iter().sum()).collect();
    let mut col = vec![0i32; m];
    for j in 0..m {
        for i in 0..n { col[j] += mat[i][j]; }
    }
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if mat[i][j] == 1 && row[i] == 1 && col[j] == 1 { ans += 1; }
        }
    }
    ans
}
fn main() { println!("{}", num_special(vec![vec![1,0,0],vec![0,1,0],vec![1,0,0]])); }
#[cfg(test)]
mod tests {
    use super::num_special;
    #[test]
    fn example_one() { assert_eq!(num_special(vec![vec![1,0,0],vec![0,1,0],vec![1,0,0]]), 1); }
}