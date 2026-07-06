/// LeetCode #2257 - Count Unguarded Cells in the Grid
fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let mut grid = vec![vec![0i32; n]; m];

    for guard in &guards {
        grid[guard[0] as usize][guard[1] as usize] = 2;
    }
    for wall in &walls {
        grid[wall[0] as usize][wall[1] as usize] = 2;
    }

    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    for guard in &guards {
        for (dx, dy) in dirs {
            let mut cx = guard[0];
            let mut cy = guard[1];
            loop {
                cx += dx;
                cy += dy;
                if cx < 0 || cx >= m as i32 || cy < 0 || cy >= n as i32 {
                    break;
                }
                let (ux, uy) = (cx as usize, cy as usize);
                if grid[ux][uy] == 2 {
                    break;
                }
                grid[ux][uy] = 1;
            }
        }
    }

    grid
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&cell| cell == 0)
        .count() as i32
}

fn main() {
    println!(
        "{}",
        count_unguarded(4, 6, vec![vec![0, 0], vec![1, 1], vec![2, 3]], vec![vec![0, 1], vec![2, 2], vec![1, 4]])
    );
}

#[cfg(test)]
mod tests {
    use super::count_unguarded;

    #[test]
    fn example_one() {
        assert_eq!(
            count_unguarded(
                4,
                6,
                vec![vec![0, 0], vec![1, 1], vec![2, 3]],
                vec![vec![0, 1], vec![2, 2], vec![1, 4]]
            ),
            7
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            count_unguarded(3, 3, vec![vec![1, 1]], vec![vec![0, 1], vec![1, 0], vec![2, 1], vec![1, 2]]),
            4
        );
    }
}
