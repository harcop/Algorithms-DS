/// LeetCode #3359 - Find Sorted Submatrices With Maximum Element at Most K
fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i64 {
    let m = grid.len();
    let n = grid[0].len();
    let mut heights = vec![0i32; m];
    let mut ans = 0i64;
    for j in 0..n {
        for i in 0..m {
            if grid[i][j] > k {
                heights[i] = 0;
            } else if j > 0 && grid[i][j] <= grid[i][j - 1] {
                heights[i] += 1;
            } else {
                heights[i] = 1;
            }
        }
        ans += count_hist(&heights);
    }
    ans
}

fn count_hist(heights: &[i32]) -> i64 {
    let mut dp = vec![0i64; heights.len()];
    let mut stk: Vec<usize> = Vec::new();
    let mut res = 0i64;
    for i in 0..heights.len() {
        while stk.last().is_some_and(|&j| heights[j] >= heights[i]) {
            stk.pop();
        }
        dp[i] = if let Some(&j) = stk.last() {
            dp[j] + heights[i] as i64 * (i - j) as i64
        } else {
            heights[i] as i64 * (i + 1) as i64
        };
        res += dp[i];
        stk.push(i);
    }
    res
}

fn main() {
    println!(
        "{}",
        count_submatrices(vec![vec![4, 3, 2, 1], vec![8, 7, 6, 1]], 3)
    );
}

#[cfg(test)]
mod tests {
    use super::count_submatrices;

    #[test]
    fn example1() {
        assert_eq!(
            count_submatrices(vec![vec![4, 3, 2, 1], vec![8, 7, 6, 1]], 3),
            8
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            count_submatrices(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]], 1),
            36
        );
    }

    #[test]
    fn example3() {
        assert_eq!(count_submatrices(vec![vec![1]], 1), 1);
    }
}
