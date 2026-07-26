/// LeetCode #2664 - The Knight's Tour
fn tour_of_knight(m: i32, n: i32, r: i32, c: i32) -> Vec<Vec<i32>> {
    let m = m as usize;
    let n = n as usize;
    let mut g = vec![vec![-1; n]; m];
    g[r as usize][c as usize] = 0;
    let dirs: [i32; 9] = [-2, -1, 2, 1, -2, 1, 2, -1, -2];
    let mut ok = false;

    fn dfs(
        i: usize,
        j: usize,
        g: &mut Vec<Vec<i32>>,
        m: usize,
        n: usize,
        dirs: &[i32; 9],
        ok: &mut bool,
    ) {
        if g[i][j] == (m * n - 1) as i32 {
            *ok = true;
            return;
        }
        for k in 0..8 {
            let x = i as i32 + dirs[k];
            let y = j as i32 + dirs[k + 1];
            if x >= 0 && (x as usize) < m && y >= 0 && (y as usize) < n {
                let (x, y) = (x as usize, y as usize);
                if g[x][y] == -1 {
                    g[x][y] = g[i][j] + 1;
                    dfs(x, y, g, m, n, dirs, ok);
                    if *ok {
                        return;
                    }
                    g[x][y] = -1;
                }
            }
        }
    }

    dfs(r as usize, c as usize, &mut g, m, n, &dirs, &mut ok);
    g
}

fn main() {
    println!("{:?}", tour_of_knight(1, 1, 0, 0));
}

#[cfg(test)]
mod tests {
    use super::tour_of_knight;

    #[test]
    fn example_one() {
        assert_eq!(tour_of_knight(1, 1, 0, 0), vec![vec![0]]);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            tour_of_knight(3, 4, 0, 0),
            vec![vec![0, 3, 6, 9], vec![11, 8, 1, 4], vec![2, 5, 10, 7]]
        );
    }
}
