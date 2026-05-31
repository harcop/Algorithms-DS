/// LeetCode #1572 - Matrix Diagonal Sum
fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
    let n = mat.len();
    let mut s = 0;
    for i in 0..n {
        s += mat[i][i];
        if i != n - 1 - i { s += mat[i][n - 1 - i]; }
    }
    s
}
fn main() { println!("{}", diagonal_sum(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]])); }
#[cfg(test)]
mod tests {
    use super::diagonal_sum;
    #[test]
    fn example_one() { assert_eq!(diagonal_sum(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]]), 25); }
    #[test]
    fn example_two() { assert_eq!(diagonal_sum(vec![vec![1,1,1,1],vec![1,1,1,1],vec![1,1,1,1],vec![1,1,1,1]]), 8); }
}