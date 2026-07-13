/// LeetCode #2373 - Largest Local Values in a Matrix
fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = grid.len();
    let mut ans = vec![vec![0; n - 2]; n - 2];
    for i in 0..n - 2 {
        for j in 0..n - 2 {
            let mut mx = 0;
            for x in i..=i + 2 {
                for y in j..=j + 2 {
                    mx = mx.max(grid[x][y]);
                }
            }
            ans[i][j] = mx;
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        largest_local(vec![
            vec![9, 9, 8, 1],
            vec![5, 6, 2, 6],
            vec![8, 2, 6, 4],
            vec![6, 2, 2, 2]
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::largest_local;

    #[test]
    fn example_one() {
        assert_eq!(
            largest_local(vec![
                vec![9, 9, 8, 1],
                vec![5, 6, 2, 6],
                vec![8, 2, 6, 4],
                vec![6, 2, 2, 2]
            ]),
            vec![vec![9, 9], vec![8, 6]]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            largest_local(vec![
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 2, 1, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1]
            ]),
            vec![vec![2, 2, 2], vec![2, 2, 2], vec![2, 2, 2]]
        );
    }
}
