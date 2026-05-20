/// LeetCode #1139 - Largest 1-Bordered Square
fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    if n == 0 {
        return 0;
    }
    let m = grid[0].len();
    let mut best = 0i32;
    for side in (1..=n.min(m)).rev() {
        'outer: for i in 0..=n - side {
            for j in 0..=m - side {
                let mut ok = true;
                for c in j..j + side {
                    if grid[i][c] != 1 || grid[i + side - 1][c] != 1 {
                        ok = false;
                        break;
                    }
                }
                if !ok {
                    continue;
                }
                for r in i..i + side {
                    if grid[r][j] != 1 || grid[r][j + side - 1] != 1 {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    best = side as i32;
                    break 'outer;
                }
            }
        }
        if best > 0 {
            break;
        }
    }
    best * best
}

fn main() {
    let grid = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    println!("{}", largest1_bordered_square(grid));
}

#[cfg(test)]
mod tests {
    use super::largest1_bordered_square;

    #[test]
    fn example_one() {
        let grid = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        assert_eq!(largest1_bordered_square(grid), 9);
    }

    #[test]
    fn example_two() {
        let grid = vec![vec![1, 1, 0, 0]];
        assert_eq!(largest1_bordered_square(grid), 1);
    }
}
