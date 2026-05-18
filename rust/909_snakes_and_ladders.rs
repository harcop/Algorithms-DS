/// LeetCode #909 - Snakes and Ladders
use std::collections::VecDeque;

fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
    let n = board.len();
    let label_to_rc = |label: i32| -> (usize, usize) {
        let label = label - 1;
        let row = n - 1 - (label as usize / n);
        let col = if (n - 1 - row) % 2 == 0 {
            label as usize % n
        } else {
            n - 1 - (label as usize % n)
        };
        (row, col)
    };

    let mut dist = vec![-1; n * n + 1];
    let mut q = VecDeque::new();
    dist[1] = 0;
    q.push_back(1);
    while let Some(x) = q.pop_front() {
        if x == (n * n) as i32 {
            return dist[x as usize];
        }
        let last = (n * n) as i32;
        for y in (x + 1)..=(x + 6).min(last) {
            let (r, c) = label_to_rc(y);
            let ny = if board[r][c] == -1 { y } else { board[r][c] };
            if dist[ny as usize] == -1 {
                dist[ny as usize] = dist[x as usize] + 1;
                q.push_back(ny);
            }
        }
    }
    -1
}

fn main() {
    println!(
        "{}",
        snakes_and_ladders(vec![
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 35, -1, -1, 13, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 15, -1, -1, -1, -1],
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::snakes_and_ladders;

    #[test]
    fn example_one() {
        let b = vec![
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 35, -1, -1, 13, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 15, -1, -1, -1, -1],
        ];
        assert_eq!(snakes_and_ladders(b), 4);
    }

    #[test]
    fn example_two() {
        let b = vec![vec![-1, -1], vec![-1, 3]];
        assert_eq!(snakes_and_ladders(b), 1);
    }
}
