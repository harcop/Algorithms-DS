/// LeetCode #1765 - Map of Highest Peak
use std::collections::VecDeque;

fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = is_water.len();
    let n = is_water[0].len();
    let mut dist = vec![vec![-1i32; n]; m];
    let mut q = VecDeque::new();
    for i in 0..m {
        for j in 0..n {
            if is_water[i][j] == 1 {
                dist[i][j] = 0;
                q.push_back((i, j));
            }
        }
    }
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    while let Some((i, j)) = q.pop_front() {
        for (di, dj) in dirs {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni >= 0
                && nj >= 0
                && (ni as usize) < m
                && (nj as usize) < n
                && dist[ni as usize][nj as usize] == -1
            {
                dist[ni as usize][nj as usize] = dist[i][j] + 1;
                q.push_back((ni as usize, nj as usize));
            }
        }
    }
    dist
}
fn main() {
    println!(
        "{:?}",
        highest_peak(vec![vec![0, 1], vec![0, 0]])
    );
}
#[cfg(test)]
mod tests {
    use super::highest_peak;
    #[test]
    fn example_one() {
        assert_eq!(highest_peak(vec![vec![0, 1], vec![0, 0]]), vec![vec![1, 0], vec![2, 1]]);
    }
    #[test]
    fn example_two() {
        assert_eq!(
            highest_peak(vec![vec![0, 0, 1], vec![1, 0, 0], vec![0, 0, 0]]),
            vec![vec![1, 1, 0], vec![0, 1, 1], vec![1, 2, 2]]
        );
    }
}
