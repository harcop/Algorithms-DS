/// LeetCode #2850 - Minimum Moves to Spread Stones Over Grid
fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
    let mut extras = Vec::new();
    let mut empty = Vec::new();
    for row in 0..3 {
        for col in 0..3 {
            if grid[row][col] == 0 {
                empty.push((row as i32, col as i32));
            } else {
                for _ in 1..grid[row][col] {
                    extras.push((row as i32, col as i32));
                }
            }
        }
    }

    fn assign(
        index: usize,
        cost: i32,
        extras: &[(i32, i32)],
        empty: &[(i32, i32)],
        used: &mut [bool],
        best: &mut i32,
    ) {
        if cost >= *best {
            return;
        }
        if index == extras.len() {
            *best = cost;
            return;
        }
        for target in 0..empty.len() {
            if !used[target] {
                used[target] = true;
                let distance = (extras[index].0 - empty[target].0).abs()
                    + (extras[index].1 - empty[target].1).abs();
                assign(index + 1, cost + distance, extras, empty, used, best);
                used[target] = false;
            }
        }
    }

    let mut best = i32::MAX;
    let mut used = vec![false; empty.len()];
    assign(0, 0, &extras, &empty, &mut used, &mut best);
    best
}

fn main() {
    println!(
        "{}",
        minimum_moves(vec![vec![1, 1, 0], vec![1, 1, 1], vec![1, 2, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_moves;

    #[test]
    fn example_one() {
        assert_eq!(
            minimum_moves(vec![vec![1, 1, 0], vec![1, 1, 1], vec![1, 2, 1]]),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            minimum_moves(vec![vec![1, 3, 0], vec![1, 0, 0], vec![1, 0, 3]]),
            4
        );
    }
}
