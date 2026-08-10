/// LeetCode #3122 - Minimum Number of Operations to Satisfy Conditions
fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let inf = 1 << 29;
    let mut f = vec![vec![inf; 10]; n];
    for i in 0..n {
        let mut cnt = [0i32; 10];
        for j in 0..m {
            cnt[grid[j][i] as usize] += 1;
        }
        if i == 0 {
            for j in 0..10 {
                f[i][j] = m as i32 - cnt[j];
            }
        } else {
            for j in 0..10 {
                for k in 0..10 {
                    if k != j {
                        f[i][j] = f[i][j].min(f[i - 1][k] + m as i32 - cnt[j]);
                    }
                }
            }
        }
    }
    *f[n - 1].iter().min().unwrap()
}

fn main() {
    println!(
        "{}",
        minimum_operations(vec![vec![1, 0, 2], vec![1, 0, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_operations;

    #[test]
    fn example1() {
        assert_eq!(
            minimum_operations(vec![vec![1, 0, 2], vec![1, 0, 2]]),
            0
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            minimum_operations(vec![vec![1, 1, 1], vec![0, 0, 0]]),
            3
        );
    }

    #[test]
    fn example3() {
        assert_eq!(minimum_operations(vec![vec![1], vec![2], vec![3]]), 2);
    }
}
