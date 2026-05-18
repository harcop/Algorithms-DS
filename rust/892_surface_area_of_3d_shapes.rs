/// LeetCode #892 - Surface Area of 3D Shapes
fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    if n == 0 {
        return 0;
    }
    let m = grid[0].len();
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            let v = grid[i][j];
            if v == 0 {
                continue;
            }
            ans += 2;
            for (di, dj) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;
                let nh = if ni >= 0 && ni < n as i32 && nj >= 0 && nj < m as i32 {
                    grid[ni as usize][nj as usize]
                } else {
                    0
                };
                ans += (v - nh).max(0);
            }
        }
    }
    ans
}

fn main() {
    println!("{}", surface_area(vec![vec![2]]));
}

#[cfg(test)]
mod tests {
    use super::surface_area;

    #[test]
    fn example_one() {
        assert_eq!(surface_area(vec![vec![2]]), 10);
    }
}
