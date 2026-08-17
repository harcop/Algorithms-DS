/// LeetCode #3240 - Minimum Number of Flips to Make Binary Grid Palindromic II
fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut ans = 0;
    for i in 0..m / 2 {
        for j in 0..n / 2 {
            let x = m - i - 1;
            let y = n - j - 1;
            let cnt1 = grid[i][j] + grid[x][j] + grid[i][y] + grid[x][y];
            ans += cnt1.min(4 - cnt1);
        }
    }
    if m % 2 == 1 && n % 2 == 1 {
        ans += grid[m / 2][n / 2];
    }
    let mut diff = 0;
    let mut cnt1 = 0;
    if m % 2 == 1 {
        for j in 0..n / 2 {
            if grid[m / 2][j] == grid[m / 2][n - j - 1] {
                cnt1 += grid[m / 2][j] * 2;
            } else {
                diff += 1;
            }
        }
    }
    if n % 2 == 1 {
        for i in 0..m / 2 {
            if grid[i][n / 2] == grid[m - i - 1][n / 2] {
                cnt1 += grid[i][n / 2] * 2;
            } else {
                diff += 1;
            }
        }
    }
    ans += if cnt1 % 4 == 0 || diff != 0 { diff } else { 2 };
    ans
}

fn main() {
    println!(
        "{}",
        min_flips(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::min_flips;

    #[test]
    fn example1() {
        assert_eq!(
            min_flips(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]),
            3
        );
    }

    #[test]
    fn example2() {
        assert_eq!(min_flips(vec![vec![0, 1], vec![0, 1], vec![0, 0]]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(min_flips(vec![vec![1], vec![1]]), 2);
    }
}
