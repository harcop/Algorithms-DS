/// LeetCode #1668 - Maximum Trailing Zeros In A Cornered Path
fn trailing(x: i32) -> (i32, i32) {
    let mut a = 0i32;
    let mut b = 0i32;
    let mut v = x;
    while v > 0 && v % 2 == 0 { a += 1; v /= 2; }
    v = x;
    while v > 0 && v % 5 == 0 { b += 1; v /= 5; }
    (a, b)
}

fn max_trailing_zeros(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut row2 = vec![vec![0i32; n + 1]; m + 1];
    let mut row5 = row2.clone();
    let mut col2 = row2.clone();
    let mut col5 = row2.clone();
    for i in 0..m {
        for j in 0..n {
            let (a, b) = trailing(grid[i][j]);
            row2[i + 1][j + 1] = row2[i + 1][j] + a;
            row5[i + 1][j + 1] = row5[i + 1][j] + b;
        }
    }
    for j in 0..n {
        for i in 0..m {
            let (a, b) = trailing(grid[i][j]);
            col2[i + 1][j + 1] = col2[i][j + 1] + a;
            col5[i + 1][j + 1] = col5[i][j + 1] + b;
        }
    }
    let mut ans = 0i32;
    for i in 0..m {
        for j in 0..n {
            let (c2, c5) = trailing(grid[i][j]);
            let t2 = row2[i + 1][j + 1] + col2[i + 1][j + 1] - c2;
            let t5 = row5[i + 1][j + 1] + col5[i + 1][j + 1] - c5;
            ans = ans.max(t2.min(t5));
            let t2 = row2[i + 1][n] - row2[i + 1][j + 1] + col2[m][j + 1] - col2[i + 1][j + 1] + c2;
            let t5 = row5[i + 1][n] - row5[i + 1][j + 1] + col5[m][j + 1] - col5[i + 1][j + 1] + c5;
            ans = ans.max(t2.min(t5));
            let t2 = row2[i + 1][j + 1] + col2[m][j + 1] - col2[i + 1][j + 1] - c2;
            let t5 = row5[i + 1][j + 1] + col5[m][j + 1] - col5[i + 1][j + 1] - c5;
            ans = ans.max(t2.min(t5));
            let t2 = row2[m][j + 1] - row2[i + 1][j + 1] + col2[i + 1][j + 1] - c2;
            let t5 = row5[m][j + 1] - row5[i + 1][j + 1] + col5[i + 1][j + 1] - c5;
            ans = ans.max(t2.min(t5));
        }
    }
    ans
}
fn main() { println!("{}", max_trailing_zeros(vec![vec![23,17,19],vec![8,1,16],vec![7,23,8],vec![1,7,1],vec![11,10,19],vec![11,28,9],vec![18,7,8],vec![26,5,4],vec![22,23,6],vec![32,29,20],vec![32,17,20],vec![32,21,4]])); }
#[cfg(test)]
mod tests {
    use super::max_trailing_zeros;
    #[test]
    fn example_one() {
        assert_eq!(max_trailing_zeros(vec![vec![23,17,19],vec![8,1,16],vec![7,23,8],vec![1,7,1],vec![11,10,19],vec![11,28,9],vec![18,7,8],vec![26,5,4],vec![22,23,6],vec![32,29,20],vec![32,17,20],vec![32,21,4]]), 3);
    }
}