/// LeetCode #840 - Magic Squares In Grid
fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
    fn valid(g: &[Vec<i32>], r: usize, c: usize) -> bool {
        let mut seen = [false; 16];
        let mut s = 0;
        for i in 0..3 {
            for j in 0..3 {
                let v = g[r + i][c + j];
                if v < 1 || v > 9 || seen[v as usize] {
                    return false;
                }
                seen[v as usize] = true;
                s += v;
            }
        }
        if s != 45 {
            return false;
        }
        for i in 0..3 {
            if g[r + i][c] + g[r + i][c + 1] + g[r + i][c + 2] != 15 {
                return false;
            }
            if g[r][c + i] + g[r + 1][c + i] + g[r + 2][c + i] != 15 {
                return false;
            }
        }
        g[r][c] + g[r + 1][c + 1] + g[r + 2][c + 2] == 15
            && g[r][c + 2] + g[r + 1][c + 1] + g[r + 2][c] == 15
    }
    let rows = grid.len();
    let cols = grid[0].len();
    let mut ans = 0;
    for r in 0..rows.saturating_sub(2) {
        for c in 0..cols.saturating_sub(2) {
            if valid(&grid, r, c) {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        num_magic_squares_inside(vec![
            vec![4, 3, 8, 4],
            vec![9, 5, 1, 9],
            vec![2, 7, 6, 2],
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::num_magic_squares_inside;

    #[test]
    fn example_one() {
        assert_eq!(
            num_magic_squares_inside(vec![
                vec![4, 3, 8, 4],
                vec![9, 5, 1, 9],
                vec![2, 7, 6, 2],
            ]),
            1
        );
    }
}
