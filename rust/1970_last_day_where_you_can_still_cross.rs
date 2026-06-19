/// LeetCode #1970 - Last Day Where You Can Still Cross
fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
    let row = row as usize;
    let col = col as usize;
    let n = row * col;

    let check = |k: usize| -> bool {
        let mut g = vec![vec![0u8; col]; row];
        for cell in cells.iter().take(k) {
            g[cell[0] as usize - 1][cell[1] as usize - 1] = 1;
        }
        let mut q: Vec<(usize, usize)> = (0..col)
            .filter(|&j| g[0][j] == 0)
            .map(|j| (0, j))
            .collect();
        let mut i = 0usize;
        while i < q.len() {
            let (x, y) = q[i];
            i += 1;
            if x == row - 1 {
                return true;
            }
            for (dx, dy) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0
                    && ny >= 0
                    && (nx as usize) < row
                    && (ny as usize) < col
                    && g[nx as usize][ny as usize] == 0
                {
                    g[nx as usize][ny as usize] = 1;
                    q.push((nx as usize, ny as usize));
                }
            }
        }
        false
    };

    let mut lo = 1usize;
    let mut hi = n;
    while lo < hi {
        let mid = lo + (hi - lo + 1) / 2;
        if check(mid) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo as i32
}

fn main() {
    println!(
        "{}",
        latest_day_to_cross(2, 2, vec![vec![1, 1], vec![2, 1], vec![1, 2], vec![2, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::latest_day_to_cross;

    #[test]
    fn example_one() {
        assert_eq!(
            latest_day_to_cross(2, 2, vec![vec![1, 1], vec![2, 1], vec![1, 2], vec![2, 2]]),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            latest_day_to_cross(2, 2, vec![vec![1, 1], vec![1, 2], vec![2, 1], vec![2, 2]]),
            1
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            latest_day_to_cross(
                3,
                3,
                vec![
                    vec![1, 2],
                    vec![2, 1],
                    vec![3, 3],
                    vec![2, 2],
                    vec![1, 1],
                    vec![1, 3],
                    vec![2, 3],
                    vec![3, 2],
                    vec![3, 1],
                ],
            ),
            3
        );
    }
}
