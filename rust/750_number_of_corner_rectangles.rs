/// LeetCode #750 - Number of Corner Rectangles
fn count_corner_rectangles(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut ans = 0i32;
    for r1 in 0..m {
        for r2 in r1 + 1..m {
            let mut c = 0i32;
            for j in 0..n {
                if grid[r1][j] == 1 && grid[r2][j] == 1 {
                    c += 1;
                }
            }
            ans += c * (c - 1) / 2;
        }
    }
    ans
}

fn main() {
    let g = vec![vec![1, 0, 0, 1, 0], vec![0, 0, 1, 0, 1], vec![0, 0, 1, 0, 1], vec![0, 0, 1, 0, 0]];
    println!("{}", count_corner_rectangles(g));
}

#[cfg(test)]
mod tests {
    use super::count_corner_rectangles;

    #[test]
    fn example_one() {
        let g = vec![vec![1, 0, 0, 1, 0], vec![0, 0, 1, 0, 1], vec![0, 0, 1, 0, 1], vec![0, 0, 1, 0, 0]];
        assert_eq!(count_corner_rectangles(g), 1);
    }
}
