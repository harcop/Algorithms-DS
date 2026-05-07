/// LeetCode #463 - Island Perimeter
fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut p = 0i32;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 0 {
                continue;
            }
            p += 4;
            if i > 0 && grid[i - 1][j] == 1 {
                p -= 2;
            }
            if j > 0 && grid[i][j - 1] == 1 {
                p -= 2;
            }
        }
    }
    p
}

fn main() {
    println!(
        "{}",
        island_perimeter(vec![vec![0, 1, 0, 0], vec![1, 1, 1, 0], vec![0, 1, 0, 0], vec![1, 1, 0, 0]])
    );
}

#[cfg(test)]
mod tests {
    use super::island_perimeter;

    #[test]
    fn example_one() {
        assert_eq!(
            island_perimeter(vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 0, 0],
                vec![1, 1, 0, 0],
            ]),
            16
        );
    }
}
