/// LeetCode #3882 - Minimum XOR Path in a Grid
fn min_xor(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut dp = vec![vec![vec![false; 1024]; n]; m];
    dp[0][0][grid[0][0] as usize] = true;
    for i in 0..m {
        for j in 0..n {
            for x in 0..1024 {
                if !dp[i][j][x] {
                    continue;
                }
                if i + 1 < m {
                    dp[i + 1][j][x ^ grid[i + 1][j] as usize] = true;
                }
                if j + 1 < n {
                    dp[i][j + 1][x ^ grid[i][j + 1] as usize] = true;
                }
            }
        }
    }
    (0..1024)
        .find(|&x| dp[m - 1][n - 1][x])
        .unwrap() as i32
}

fn main() {
    println!("{}", min_xor(vec![vec![1, 2], vec![3, 4]]));
}

#[cfg(test)]
mod tests {
    use super::min_xor;

    #[test]
    fn example1() {
        assert_eq!(min_xor(vec![vec![1, 2], vec![3, 4]]), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(min_xor(vec![vec![6, 7], vec![5, 8]]), 9);
    }

    #[test]
    fn example3() {
        assert_eq!(min_xor(vec![vec![2, 7, 5]]), 0);
    }
}
