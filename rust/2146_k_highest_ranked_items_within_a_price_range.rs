/// LeetCode #2146 - K Highest Ranked Items Within a Price Range
use std::collections::VecDeque;

fn highest_ranked_k_items(
    grid: Vec<Vec<i32>>,
    pricing: Vec<i32>,
    start: Vec<i32>,
    k: i32,
) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len();
    let low = pricing[0];
    let high = pricing[1];
    let sr = start[0] as usize;
    let sc = start[1] as usize;

    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut visited = vec![vec![false; n]; m];
    let mut queue = VecDeque::new();
    queue.push_back((sr, sc, 0));
    visited[sr][sc] = true;

    let mut items = Vec::new();
    while let Some((r, c, dist)) = queue.pop_front() {
        let price = grid[r][c];
        if price >= low && price <= high {
            items.push((dist, price, r as i32, c as i32));
        }

        for (dr, dc) in dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0 || nc < 0 || nr as usize >= m || nc as usize >= n {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if visited[nr][nc] || grid[nr][nc] == 0 {
                continue;
            }
            visited[nr][nc] = true;
            queue.push_back((nr, nc, dist + 1));
        }
    }

    items.sort_unstable();
    items
        .into_iter()
        .take(k as usize)
        .map(|(_, _, r, c)| vec![r, c])
        .collect()
}

fn main() {
    println!(
        "{:?}",
        highest_ranked_k_items(
            vec![vec![1, 2, 0, 1], vec![1, 3, 0, 1], vec![0, 2, 5, 1]],
            vec![2, 5],
            vec![0, 0],
            3
        )
    );
}

#[cfg(test)]
mod tests {
    use super::highest_ranked_k_items;

    #[test]
    fn example_one() {
        assert_eq!(
            highest_ranked_k_items(
                vec![vec![1, 2, 0, 1], vec![1, 3, 0, 1], vec![0, 2, 5, 1]],
                vec![2, 5],
                vec![0, 0],
                3
            ),
            vec![vec![0, 1], vec![1, 1], vec![2, 1]]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            highest_ranked_k_items(
                vec![vec![1, 2, 0, 1], vec![1, 3, 3, 1], vec![0, 2, 5, 1]],
                vec![2, 3],
                vec![2, 3],
                2
            ),
            vec![vec![2, 1], vec![1, 2]]
        );
    }
}
