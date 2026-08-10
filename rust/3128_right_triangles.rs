/// LeetCode #3128 - Right Triangles
fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
    let m = grid.len();
    let n = grid[0].len();
    let mut rows = vec![0i64; m];
    let mut cols = vec![0i64; n];
    for i in 0..m {
        for j in 0..n {
            rows[i] += grid[i][j] as i64;
            cols[j] += grid[i][j] as i64;
        }
    }
    let mut ans = 0i64;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                ans += (rows[i] - 1) * (cols[j] - 1);
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        number_of_right_triangles(vec![vec![0, 1, 0], vec![0, 1, 1], vec![0, 1, 0]])
    );
}

#[cfg(test)]
mod tests {
    use super::number_of_right_triangles;

    #[test]
    fn example1() {
        assert_eq!(
            number_of_right_triangles(vec![vec![0, 1, 0], vec![0, 1, 1], vec![0, 1, 0]]),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            number_of_right_triangles(vec![vec![1, 0, 0, 0], vec![0, 1, 0, 1], vec![1, 0, 0, 0]]),
            0
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            number_of_right_triangles(vec![vec![1, 0, 1], vec![1, 0, 0], vec![1, 0, 0]]),
            2
        );
    }
}
