/// LeetCode #1536 - Minimum Swaps To Arrange A Binary Grid
fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut need = vec![0; n];
    for i in 0..n {
        let mut zeros = 0;
        for j in (0..n).rev() {
            if grid[i][j] == 1 { break; }
            zeros += 1;
        }
        need[i] = zeros;
    }
    let mut ans = 0;
    for i in 0..n {
        let req = n - 1 - i;
        let mut j = i;
        while j < n && need[j] < req { j += 1; }
        if j == n { return -1; }
        while j > i { need.swap(j, j - 1); ans += 1; j -= 1; }
    }
    ans
}
fn main() { println!("{}", min_swaps(vec![vec![0, 0, 1], vec![1, 1, 0], vec![1, 0, 0]])); }
#[cfg(test)]
mod tests {
    use super::min_swaps;
    #[test]
    fn example_one() { assert_eq!(min_swaps(vec![vec![0, 0, 1], vec![1, 1, 0], vec![1, 0, 0]]), 3); }
    #[test]
    fn example_two() { assert_eq!(min_swaps(vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 1, 1, 0]]), -1); }
}
