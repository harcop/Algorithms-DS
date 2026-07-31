/// LeetCode #2814 - Minimum Time Takes to Reach Destination Without Drowning
use std::collections::VecDeque;

fn minimum_seconds(land: Vec<Vec<char>>) -> i32 {
    let m = land.len();
    let n = land[0].len();
    let inf = i32::MAX / 4;
    let mut flood = vec![vec![inf; n]; m];
    let mut vis = vec![vec![false; n]; m];
    let mut q = VecDeque::new();
    let mut si = 0;
    let mut sj = 0;

    for i in 0..m {
        for j in 0..n {
            match land[i][j] {
                '*' => q.push_back((i, j)),
                'S' => {
                    si = i;
                    sj = j;
                }
                _ => {}
            }
        }
    }

    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut t = 0;
    while !q.is_empty() {
        for _ in 0..q.len() {
            let (i, j) = q.pop_front().unwrap();
            flood[i][j] = t;
            for (di, dj) in dirs {
                let x = i as i32 + di;
                let y = j as i32 + dj;
                if x >= 0 && x < m as i32 && y >= 0 && y < n as i32 {
                    let (x, y) = (x as usize, y as usize);
                    if !vis[x][y] && (land[x][y] == '.' || land[x][y] == 'S') {
                        vis[x][y] = true;
                        q.push_back((x, y));
                    }
                }
            }
        }
        t += 1;
    }

    vis = vec![vec![false; n]; m];
    q.push_back((si, sj));
    vis[si][sj] = true;
    t = 0;
    while !q.is_empty() {
        for _ in 0..q.len() {
            let (i, j) = q.pop_front().unwrap();
            if land[i][j] == 'D' {
                return t;
            }
            for (di, dj) in dirs {
                let x = i as i32 + di;
                let y = j as i32 + dj;
                if x >= 0 && x < m as i32 && y >= 0 && y < n as i32 {
                    let (x, y) = (x as usize, y as usize);
                    if !vis[x][y]
                        && flood[x][y] > t + 1
                        && (land[x][y] == '.' || land[x][y] == 'D')
                    {
                        vis[x][y] = true;
                        q.push_back((x, y));
                    }
                }
            }
        }
        t += 1;
    }
    -1
}

fn main() {
    let land = vec![
        vec!['D', '.', '*'],
        vec!['.', '.', '.'],
        vec!['.', 'S', '.'],
    ];
    println!("{}", minimum_seconds(land));
}

#[cfg(test)]
mod tests {
    use super::minimum_seconds;

    #[test]
    fn example_one() {
        let land = vec![
            vec!['D', '.', '*'],
            vec!['.', '.', '.'],
            vec!['.', 'S', '.'],
        ];
        assert_eq!(minimum_seconds(land), 3);
    }

    #[test]
    fn example_two() {
        let land = vec![
            vec!['D', 'X', '*'],
            vec!['.', '.', '.'],
            vec!['.', '.', 'S'],
        ];
        assert_eq!(minimum_seconds(land), -1);
    }

    #[test]
    fn example_three() {
        let land = vec![
            vec!['D', '.', '.', '.', '*', '.'],
            vec!['.', 'X', '.', 'X', '.', '.'],
            vec!['.', '.', '.', '.', 'S', '.'],
        ];
        assert_eq!(minimum_seconds(land), 6);
    }
}
