/// LeetCode #2132 - Stamping the Grid
fn possible_to_stamp(grid: Vec<Vec<i32>>, stamp_height: i32, stamp_width: i32) -> bool {
    let m = grid.len();
    let n = grid[0].len();
    let h = stamp_height as usize;
    let w = stamp_width as usize;

    let mut ones = vec![vec![0; n + 1]; m + 1];
    for i in 0..m {
        for j in 0..n {
            ones[i + 1][j + 1] = grid[i][j] + ones[i][j + 1] + ones[i + 1][j] - ones[i][j];
        }
    }

    let mut diff = vec![vec![0; n + 1]; m + 1];
    if h <= m && w <= n {
        for i in 0..=m - h {
            for j in 0..=n - w {
                let blocked = ones[i + h][j + w] - ones[i][j + w] - ones[i + h][j] + ones[i][j];
                if blocked == 0 {
                    diff[i][j] += 1;
                    diff[i + h][j] -= 1;
                    diff[i][j + w] -= 1;
                    diff[i + h][j + w] += 1;
                }
            }
        }
    }

    for i in 0..m {
        for j in 0..n {
            let top = if i > 0 { diff[i - 1][j] } else { 0 };
            let left = if j > 0 { diff[i][j - 1] } else { 0 };
            let corner = if i > 0 && j > 0 { diff[i - 1][j - 1] } else { 0 };
            diff[i][j] += top + left - corner;

            if grid[i][j] == 0 && diff[i][j] == 0 {
                return false;
            }
        }
    }

    true
}

fn main() {
    println!(
        "{}",
        possible_to_stamp(
            vec![
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 0]
            ],
            4,
            3
        )
    );
}

#[cfg(test)]
mod tests {
    use super::possible_to_stamp;

    #[test]
    fn example_one() {
        assert!(possible_to_stamp(
            vec![
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 0]
            ],
            4,
            3
        ));
    }

    #[test]
    fn example_two() {
        assert!(!possible_to_stamp(
            vec![
                vec![1, 0, 0, 0],
                vec![0, 1, 0, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 0, 1]
            ],
            2,
            2
        ));
    }
}
