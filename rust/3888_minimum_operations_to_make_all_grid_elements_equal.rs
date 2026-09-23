/// LeetCode #3888 - Minimum Operations to Make All Grid Elements Equal
fn min_operations(grid: Vec<Vec<i32>>, k: i32) -> i64 {
    let k = k as usize;
    let mut mx = 0i32;
    for row in &grid {
        for &v in row {
            mx = mx.max(v);
        }
    }

    fn try_target(grid: &[Vec<i32>], k: usize, target: i64) -> Option<i64> {
        let m = grid.len();
        let n = grid[0].len();
        let mut diff = vec![vec![0i64; n + 1]; m + 1];
        let mut add = vec![vec![0i64; n]; m];
        let mut ops = 0i64;
        for i in 0..m {
            for j in 0..n {
                add[i][j] = diff[i][j];
                if i > 0 {
                    add[i][j] += add[i - 1][j];
                }
                if j > 0 {
                    add[i][j] += add[i][j - 1];
                }
                if i > 0 && j > 0 {
                    add[i][j] -= add[i - 1][j - 1];
                }
                let cur = grid[i][j] as i64 + add[i][j];
                if cur > target {
                    return None;
                }
                let need = target - cur;
                ops += need;
                if need > 0 {
                    if i + k > m || j + k > n {
                        return None;
                    }
                    add[i][j] += need;
                    diff[i][j] += need;
                    diff[i][j + k] -= need;
                    diff[i + k][j] -= need;
                    diff[i + k][j + k] += need;
                }
            }
        }
        Some(ops)
    }

    let mut ans = i64::MAX;
    for &target in &[mx as i64, mx as i64 + 1] {
        if let Some(v) = try_target(&grid, k, target) {
            ans = ans.min(v);
        }
    }
    if ans == i64::MAX {
        -1
    } else {
        ans
    }
}

fn main() {
    println!(
        "{}",
        min_operations(vec![vec![3, 3, 5], vec![3, 3, 5]], 2)
    );
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(
            min_operations(vec![vec![3, 3, 5], vec![3, 3, 5]], 2),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![vec![1, 2], vec![2, 3]], 1), 4);
    }
}
