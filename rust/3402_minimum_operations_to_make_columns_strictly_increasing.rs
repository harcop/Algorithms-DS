/// LeetCode #3402 - Minimum Operations to Make Columns Strictly Increasing
fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
    if grid.is_empty() {
        return 0;
    }
    let m = grid.len();
    let n = grid[0].len();
    let mut ans = 0;
    for j in 0..n {
        let mut pre = -1;
        for i in 0..m {
            let cur = grid[i][j];
            if pre < cur {
                pre = cur;
            } else {
                pre += 1;
                ans += pre - cur;
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        minimum_operations(vec![vec![3, 2], vec![1, 3], vec![3, 4], vec![0, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_operations;

    #[test]
    fn example1() {
        assert_eq!(
            minimum_operations(vec![vec![3, 2], vec![1, 3], vec![3, 4], vec![0, 1]]),
            15
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            minimum_operations(vec![vec![3, 2, 1], vec![2, 1, 0], vec![1, 2, 3]]),
            12
        );
    }
}
