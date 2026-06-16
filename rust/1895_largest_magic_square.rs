/// LeetCode #1895 - Largest Magic Square
fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut rowsum = vec![vec![0i64; n + 1]; m + 1];
    let mut colsum = vec![vec![0i64; n + 1]; m + 1];
    for i in 0..m {
        for j in 0..n {
            rowsum[i + 1][j + 1] = rowsum[i + 1][j] + grid[i][j] as i64;
            colsum[i + 1][j + 1] = colsum[i][j + 1] + grid[i][j] as i64;
        }
    }

    let check = |x1: usize, y1: usize, x2: usize, y2: usize| -> bool {
        let val = rowsum[x2 + 1][y2 + 1] - rowsum[x2 + 1][y1];
        for i in x1 + 1..=x2 {
            if rowsum[i + 1][y2 + 1] - rowsum[i + 1][y1] != val {
                return false;
            }
        }
        for j in y1..=y2 {
            if colsum[x2 + 1][j + 1] - colsum[x1][j + 1] != val {
                return false;
            }
        }
        let mut s = 0i64;
        let (mut i, mut j) = (x1, y1);
        while i <= x2 {
            s += grid[i][j] as i64;
            i += 1;
            j += 1;
        }
        if s != val {
            return false;
        }
        s = 0;
        i = x1;
        j = y2;
        while i <= x2 {
            s += grid[i][j] as i64;
            i += 1;
            j = j.saturating_sub(1);
        }
        s == val
    };

    for k in (2..=m.min(n)).rev() {
        for i in 0..=m - k {
            for j in 0..=n - k {
                if check(i, j, i + k - 1, j + k - 1) {
                    return k as i32;
                }
            }
        }
    }
    1
}

fn main() {
    let grid = vec![
        vec![7, 1, 4, 5, 6],
        vec![2, 5, 1, 6, 4],
        vec![1, 5, 4, 3, 2],
        vec![1, 2, 7, 3, 4],
    ];
    println!("{}", largest_magic_square(grid));
}

#[cfg(test)]
mod tests {
    use super::largest_magic_square;

    #[test]
    fn example_one() {
        let grid = vec![
            vec![7, 1, 4, 5, 6],
            vec![2, 5, 1, 6, 4],
            vec![1, 5, 4, 3, 2],
            vec![1, 2, 7, 3, 4],
        ];
        assert_eq!(largest_magic_square(grid), 3);
    }
}
