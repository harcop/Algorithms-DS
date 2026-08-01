use std::collections::VecDeque;

/// LeetCode #2852 - Sum of Remoteness of All Cells
fn sum_remoteness(grid: Vec<Vec<i64>>) -> i64 {
    let rows = grid.len();
    let cols = grid[0].len();
    let total: i64 = grid
        .iter()
        .flatten()
        .copied()
        .filter(|&value| value != -1)
        .sum();
    let mut visited = vec![vec![false; cols]; rows];
    let mut answer = 0;

    for start_row in 0..rows {
        for start_col in 0..cols {
            if grid[start_row][start_col] == -1 || visited[start_row][start_col] {
                continue;
            }
            let mut queue = VecDeque::from([(start_row, start_col)]);
            visited[start_row][start_col] = true;
            let mut size = 0;
            let mut component_sum = 0;

            while let Some((row, col)) = queue.pop_front() {
                size += 1;
                component_sum += grid[row][col];
                for (dr, dc) in [(1isize, 0isize), (-1, 0), (0, 1), (0, -1)] {
                    let next_row = row as isize + dr;
                    let next_col = col as isize + dc;
                    if next_row < 0
                        || next_col < 0
                        || next_row >= rows as isize
                        || next_col >= cols as isize
                    {
                        continue;
                    }
                    let (next_row, next_col) = (next_row as usize, next_col as usize);
                    if grid[next_row][next_col] != -1 && !visited[next_row][next_col] {
                        visited[next_row][next_col] = true;
                        queue.push_back((next_row, next_col));
                    }
                }
            }
            answer += size * (total - component_sum);
        }
    }
    answer
}

fn main() {
    println!(
        "{}",
        sum_remoteness(vec![vec![-1, 1, -1], vec![5, -1, 4], vec![-1, 3, -1]])
    );
}

#[cfg(test)]
mod tests {
    use super::sum_remoteness;

    #[test]
    fn example_one() {
        assert_eq!(
            sum_remoteness(vec![
                vec![-1, 1, -1],
                vec![5, -1, 4],
                vec![-1, 3, -1]
            ]),
            39
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            sum_remoteness(vec![
                vec![-1, 3, 4],
                vec![-1, -1, -1],
                vec![3, -1, -1]
            ]),
            13
        );
    }
}
