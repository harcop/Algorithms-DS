/// LeetCode #1559 - Detect Cycles In 2D Grid
fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
    let n = grid.len();
    if n < 2 {
        return false;
    }
    let m = grid[0].len();
    if m < 2 {
        return false;
    }
    for r in 0..n - 1 {
        for c in 0..m - 1 {
            let ch = grid[r][c];
            if grid[r][c + 1] == ch && grid[r + 1][c] == ch && grid[r + 1][c + 1] == ch {
                return true;
            }
        }
    }
    false
}

fn main() {
    println!("{}", contains_cycle(vec![vec!['a', 'a'], vec!['a', 'a']]));
}

#[cfg(test)]
mod tests {
    use super::contains_cycle;

    #[test]
    fn example_one() {
        assert!(contains_cycle(vec![vec!['a', 'a'], vec!['a', 'a']]));
    }

    #[test]
    fn example_two() {
        assert!(!contains_cycle(vec![
            vec!['c', 'c', 'c', 'a'],
            vec!['c', 'd', 'c', 'c'],
            vec!['c', 'c', 'e', 'c'],
            vec!['f', 'c', 'c', 'c'],
        ]));
    }
}
