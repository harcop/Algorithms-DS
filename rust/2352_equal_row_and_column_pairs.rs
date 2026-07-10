/// LeetCode #2352 - Equal Row and Column Pairs
fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            let mut ok = 1;
            for k in 0..n {
                if grid[i][k] != grid[k][j] {
                    ok = 0;
                    break;
                }
            }
            ans += ok;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        equal_pairs(vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]])
    );
}

#[cfg(test)]
mod tests {
    use super::equal_pairs;

    #[test]
    fn example_one() {
        assert_eq!(
            equal_pairs(vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]]),
            1
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            equal_pairs(vec![
                vec![3, 1, 2, 2],
                vec![1, 4, 4, 5],
                vec![2, 4, 2, 2],
                vec![2, 4, 2, 2]
            ]),
            3
        );
    }
}
