/// LeetCode #864 - Shortest Path to Get All Keys
use std::collections::{HashSet, VecDeque};

fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
    let g: Vec<Vec<char>> = grid.iter().map(|r| r.chars().collect()).collect();
    let m = g.len();
    let n = g[0].len();
    let mut start = (0, 0);
    let mut keys = 0;
    for i in 0..m {
        for j in 0..n {
            if g[i][j] == '@' {
                start = (i, j);
            } else if g[i][j].is_ascii_lowercase() {
                keys |= 1 << (g[i][j] as u8 - b'a');
            }
        }
    }
    let all = keys;
    let mut seen = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((start.0, start.1, 0u32, 0i32));
    seen.insert((start.0, start.1, 0u32));
    let dirs = [(0isize, 1isize), (0, -1), (1, 0), (-1, 0)];
    while let Some((r, c, mask, dist)) = q.pop_front() {
        if mask == all {
            return dist;
        }
        for (dr, dc) in dirs {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr < 0 || nc < 0 || nr >= m as isize || nc >= n as isize {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;
            let ch = g[nr][nc];
            if ch == '#' {
                continue;
            }
            if ch.is_ascii_uppercase() {
                let need = 1u32 << (ch as u8 - b'A');
                if mask & need == 0 {
                    continue;
                }
            }
            let mut nmask = mask;
            if ch.is_ascii_lowercase() {
                nmask |= 1 << (ch as u8 - b'a');
            }
            if seen.insert((nr, nc, nmask)) {
                q.push_back((nr, nc, nmask, dist + 1));
            }
        }
    }
    -1
}

fn main() {
    let grid = vec!["@.a..".into(), "###.#".into(), "b.A.B".into()];
    println!("{}", shortest_path_all_keys(grid));
}

#[cfg(test)]
mod tests {
    use super::shortest_path_all_keys;

    #[test]
    fn example_one() {
        let grid = vec!["@.a..".into(), "###.#".into(), "b.A.B".into()];
        assert_eq!(shortest_path_all_keys(grid), 8);
    }

    #[test]
    fn example_two() {
        let grid = vec!["@..aA".into(), "..B#.".into(), "....b".into()];
        assert_eq!(shortest_path_all_keys(grid), 6);
    }

    #[test]
    fn example_three() {
        let grid = vec!["@Aa".into()];
        assert_eq!(shortest_path_all_keys(grid), -1);
    }
}
