/// LeetCode #542 - 01 Matrix
use std::collections::VecDeque;

fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = mat.len();
    let n = mat[0].len();
    let mut dist = vec![vec![i32::MAX / 4; n]; m];
    let mut q = VecDeque::new();
    for i in 0..m {
        for j in 0..n {
            if mat[i][j] == 0 {
                dist[i][j] = 0;
                q.push_back((i, j));
            }
        }
    }
    while let Some((i, j)) = q.pop_front() {
        let d = dist[i][j] + 1;
        for (di, dj) in [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)] {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 {
                let ni = ni as usize;
                let nj = nj as usize;
                if dist[ni][nj] > d {
                    dist[ni][nj] = d;
                    q.push_back((ni, nj));
                }
            }
        }
    }
    dist
}

fn main() {
    println!("{:?}", update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]));
}

#[cfg(test)]
mod tests {
    use super::update_matrix;

    #[test]
    fn example_one() {
        assert_eq!(
            update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]]
        );
    }
}
