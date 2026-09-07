/// LeetCode #3619 - Count Islands With Total Value Divisible by K
fn count_islands(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let k = k as i64;
    let mut ans = 0;
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] > 0 {
                let mut s = 0i64;
                let mut stack = vec![(i, j)];
                s += grid[i][j] as i64;
                grid[i][j] = 0;
                while let Some((x, y)) = stack.pop() {
                    for (dx, dy) in dirs {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        if nx >= 0 && ny >= 0 {
                            let (ux, uy) = (nx as usize, ny as usize);
                            if ux < m && uy < n && grid[ux][uy] > 0 {
                                s += grid[ux][uy] as i64;
                                grid[ux][uy] = 0;
                                stack.push((ux, uy));
                            }
                        }
                    }
                }
                if s % k == 0 {
                    ans += 1;
                }
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        count_islands(
            vec![
                vec![0, 2, 1, 0, 0],
                vec![0, 5, 0, 0, 5],
                vec![0, 0, 1, 0, 0],
                vec![0, 1, 4, 7, 0],
                vec![0, 2, 0, 0, 8]
            ],
            5
        )
    );
}

#[cfg(test)]
mod tests {
    use super::count_islands;

    #[test]
    fn example1() {
        assert_eq!(
            count_islands(
                vec![
                    vec![0, 2, 1, 0, 0],
                    vec![0, 5, 0, 0, 5],
                    vec![0, 0, 1, 0, 0],
                    vec![0, 1, 4, 7, 0],
                    vec![0, 2, 0, 0, 8]
                ],
                5
            ),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            count_islands(vec![vec![3, 0, 3, 0], vec![0, 3, 0, 3], vec![3, 0, 3, 0]], 3),
            6
        );
    }
}
