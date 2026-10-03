/// LeetCode #4052 - Cyclically Shift Rows and Columns
fn cyclic_shift(n: i32, grid: Vec<Vec<i32>>, row_shift: Vec<i32>, col_shift: Vec<i32>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut shifted = vec![vec![0; n]; n];
    for i in 0..n {
        let shift = row_shift[i] as usize;
        for j in 0..n {
            shifted[i][(j + n - shift) % n] = grid[i][j];
        }
    }
    let mut ans = vec![vec![0; n]; n];
    for j in 0..n {
        let shift = col_shift[j] as usize;
        for i in 0..n {
            ans[(i + n - shift) % n][j] = shifted[i][j];
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        cyclic_shift(2, vec![vec![1, 2], vec![3, 4]], vec![1, 0], vec![0, 1])
    );
}

#[cfg(test)]
mod tests {
    use super::cyclic_shift;

    #[test]
    fn example1() {
        assert_eq!(
            cyclic_shift(2, vec![vec![1, 2], vec![3, 4]], vec![1, 0], vec![0, 1]),
            vec![vec![2, 4], vec![3, 1]]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            cyclic_shift(
                3,
                vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
                vec![1, 2, 0],
                vec![2, 2, 1]
            ),
            vec![vec![7, 8, 5], vec![2, 3, 9], vec![6, 4, 1]]
        );
    }
}
