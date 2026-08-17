/// LeetCode #3239 - Minimum Number of Flips to Make Binary Grid Palindromic I
fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut cnt1 = 0;
    let mut cnt2 = 0;
    for row in &grid {
        for j in 0..n / 2 {
            if row[j] != row[n - j - 1] {
                cnt1 += 1;
            }
        }
    }
    for j in 0..n {
        for i in 0..m / 2 {
            if grid[i][j] != grid[m - i - 1][j] {
                cnt2 += 1;
            }
        }
    }
    cnt1.min(cnt2)
}

fn main() {
    println!(
        "{}",
        min_flips(vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::min_flips;

    #[test]
    fn example1() {
        assert_eq!(
            min_flips(vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 1]]),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(min_flips(vec![vec![0, 1], vec![0, 1], vec![0, 0]]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(min_flips(vec![vec![1], vec![0]]), 0);
    }
}
