/// LeetCode #2017 - Grid Game
fn grid_game(grid: Vec<Vec<i64>>) -> i64 {
    let mut ans = i64::MAX;
    let mut s1: i64 = grid[0].iter().sum();
    let mut s2 = 0i64;
    for (j, &v) in grid[0].iter().enumerate() {
        s1 -= v;
        ans = ans.min(s1.max(s2));
        s2 += grid[1][j];
    }
    ans
}

fn main() {
    println!(
        "{}",
        grid_game(vec![vec![2, 5, 4], vec![1, 5, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::grid_game;

    #[test]
    fn example_one() {
        assert_eq!(grid_game(vec![vec![2, 5, 4], vec![1, 5, 1]]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(grid_game(vec![vec![3, 3, 1], vec![8, 5, 2]]), 4);
    }

    #[test]
    fn example_three() {
        assert_eq!(grid_game(vec![vec![1, 3, 1, 15], vec![1, 3, 3, 1]]), 7);
    }
}
