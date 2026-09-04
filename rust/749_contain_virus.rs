/// LeetCode #749 - Contain Virus
use std::collections::{HashSet, VecDeque};

fn contain_virus(mut is_infected: Vec<Vec<i32>>) -> i32 {
    if is_infected.is_empty() {
        return 0;
    }
    let m = is_infected.len();
    let n = is_infected[0].len();
    let dirs = [(0isize, 1isize), (0, -1), (1, 0), (-1, 0)];
    let mut walls = 0;

    loop {
        let mut seen = vec![vec![false; n]; m];
        let mut regions: Vec<(Vec<(usize, usize)>, HashSet<(usize, usize)>, i32)> = Vec::new();

        for i in 0..m {
            for j in 0..n {
                if is_infected[i][j] == 1 && !seen[i][j] {
                    let mut cells = Vec::new();
                    let mut threatened = HashSet::new();
                    let mut need = 0;
                    let mut q = VecDeque::new();
                    q.push_back((i, j));
                    seen[i][j] = true;
                    while let Some((r, c)) = q.pop_front() {
                        cells.push((r, c));
                        for (dr, dc) in dirs {
                            let nr = r as isize + dr;
                            let nc = c as isize + dc;
                            if nr < 0 || nc < 0 || nr >= m as isize || nc >= n as isize {
                                continue;
                            }
                            let nr = nr as usize;
                            let nc = nc as usize;
                            if is_infected[nr][nc] == 1 && !seen[nr][nc] {
                                seen[nr][nc] = true;
                                q.push_back((nr, nc));
                            } else if is_infected[nr][nc] == 0 {
                                threatened.insert((nr, nc));
                                need += 1;
                            }
                        }
                    }
                    regions.push((cells, threatened, need));
                }
            }
        }

        if regions.is_empty() {
            break;
        }
        let mut best = 0;
        for (idx, (_, thr, _)) in regions.iter().enumerate() {
            if thr.len() > regions[best].1.len() {
                best = idx;
            }
        }
        if regions[best].1.is_empty() {
            break;
        }

        walls += regions[best].2;
        for (r, c) in &regions[best].0 {
            is_infected[*r][*c] = -1;
        }

        for (idx, (_, thr, _)) in regions.iter().enumerate() {
            if idx == best {
                continue;
            }
            for &(r, c) in thr {
                is_infected[r][c] = 1;
            }
        }
    }
    walls
}

fn main() {
    let g = vec![
        vec![0, 1, 0, 0, 0, 0, 0, 1],
        vec![0, 1, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
    ];
    println!("{}", contain_virus(g));
}

#[cfg(test)]
mod tests {
    use super::contain_virus;

    #[test]
    fn example_one() {
        let g = vec![
            vec![0, 1, 0, 0, 0, 0, 0, 1],
            vec![0, 1, 0, 0, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
        ];
        assert_eq!(contain_virus(g), 10);
    }

    #[test]
    fn example_two() {
        let g = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        assert_eq!(contain_virus(g), 4);
    }

    #[test]
    fn example_three() {
        let g = vec![
            vec![1, 1, 1, 0, 0, 0, 0, 0, 0],
            vec![1, 0, 1, 0, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 0, 0, 0, 0, 0, 0],
        ];
        assert_eq!(contain_virus(g), 13);
    }
}
