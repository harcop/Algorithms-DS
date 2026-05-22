/// LeetCode #1197 - Minimum Knight Moves
fn min_knight_moves(x: i32, y: i32) -> i32 {
    let x = x.abs();
    let y = y.abs();
    if x == 0 && y == 0 {
        return 0;
    }
    let dirs = [(2, 1), (1, 2), (-1, 2), (-2, 1), (-2, -1), (-1, -2), (1, -2), (2, -1)];
    let mut q = std::collections::VecDeque::from([(0, 0, 0)]);
    let mut seen = std::collections::HashSet::from([(0, 0)]);
    while let Some((cx, cy, d)) = q.pop_front() {
        if cx == x && cy == y {
            return d;
        }
        for (dx, dy) in dirs {
            let nx = cx + dx;
            let ny = cy + dy;
            if nx >= -1 && ny >= -1 && nx <= x + 1 && ny <= y + 1 && seen.insert((nx, ny)) {
                q.push_back((nx, ny, d + 1));
            }
        }
    }
    -1
}

fn main() {
    println!("{}", min_knight_moves(2, 1));
}

#[cfg(test)]
mod tests {
    use super::min_knight_moves;

    #[test]
    fn example_one() {
        assert_eq!(min_knight_moves(2, 1), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_knight_moves(5, 5), 4);
    }
}
