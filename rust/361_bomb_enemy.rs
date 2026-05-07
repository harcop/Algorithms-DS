/// LeetCode #361 - Bomb Enemy
fn max_killed_enemies(grid: Vec<Vec<char>>) -> i32 {
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();
    let mut best = 0i32;
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != '0' {
                continue;
            }
            let mut kills = 0i32;
            let mut rr = r;
            while rr > 0 && grid[rr - 1][c] != 'W' {
                rr -= 1;
                if grid[rr][c] == 'E' {
                    kills += 1;
                }
            }
            rr = r;
            while rr + 1 < rows && grid[rr + 1][c] != 'W' {
                rr += 1;
                if grid[rr][c] == 'E' {
                    kills += 1;
                }
            }
            let mut cc = c;
            while cc > 0 && grid[r][cc - 1] != 'W' {
                cc -= 1;
                if grid[r][cc] == 'E' {
                    kills += 1;
                }
            }
            cc = c;
            while cc + 1 < cols && grid[r][cc + 1] != 'W' {
                cc += 1;
                if grid[r][cc] == 'E' {
                    kills += 1;
                }
            }
            best = best.max(kills);
        }
    }
    best
}

fn main() {
    let g = vec![
        vec!['0', 'E', '0', '0'],
        vec!['E', '0', 'W', 'E'],
        vec!['0', 'E', '0', '0'],
    ];
    println!("{}", max_killed_enemies(g));
}

#[cfg(test)]
mod tests {
    use super::max_killed_enemies;

    #[test]
    fn ex() {
        let g = vec![
            vec!['0', 'E', '0', '0'],
            vec!['E', '0', 'W', 'E'],
            vec!['0', 'E', '0', '0'],
        ];
        assert_eq!(max_killed_enemies(g), 3);
    }
}
