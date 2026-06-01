/// LeetCode #1706 - Where Will The Ball Fall
fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let m = grid.len();
    let n = grid[0].len();
    let mut ans = vec![-1; n];
    for c in 0..n {
        let mut col = c as i32;
        let mut row = 0i32;
        loop {
            if row < 0 || row >= m as i32 { break; }
            let d = grid[row as usize][col as usize];
            let nc = col + d;
            if nc < 0 || nc >= n as i32 || grid[row as usize][nc as usize] != d {
                col = -1;
                break;
            }
            row += 1;
            col = nc;
        }
        if row == m as i32 { ans[c] = col; }
    }
    ans
}
fn main() { println!("{:?}", find_ball(vec![vec![1,1,1,-1,-1],vec![1,1,1,-1,-1],vec![-1,-1,-1,1,1],vec![1,1,1,1,-1],vec![-1,-1,-1,-1,-1]])); }
#[cfg(test)]
mod tests {
    use super::find_ball;
    #[test]
    fn example_one() { assert_eq!(find_ball(vec![vec![1,1,1,-1,-1],vec![1,1,1,-1,-1],vec![-1,-1,-1,1,1],vec![1,1,1,1,-1],vec![-1,-1,-1,-1,-1]]), vec![1,-1,-1,-1,-1]); }
}