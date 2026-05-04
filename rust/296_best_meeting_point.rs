/// LeetCode #296 - Best Meeting Point
fn min_total_distance(grid: Vec<Vec<i32>>) -> i32 {
    let mut xs = vec![];
    let mut ys = vec![];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 1 {
                xs.push(i as i32);
                ys.push(j as i32);
            }
        }
    }
    xs.sort_unstable();
    ys.sort_unstable();
    let mx = xs[xs.len() / 2];
    let my = ys[ys.len() / 2];
    xs.iter().map(|&x| (x - mx).abs()).sum::<i32>()
        + ys.iter().map(|&y| (y - my).abs()).sum::<i32>()
}

fn main() {
    println!(
        "{}",
        min_total_distance(vec![vec![1, 0, 0, 0, 1], vec![0, 0, 0, 0, 0], vec![0, 0, 1, 0, 0]])
    );
}

#[cfg(test)]
mod tests {
    use super::min_total_distance;

    #[test]
    fn example_one() {
        assert_eq!(
            min_total_distance(vec![vec![1, 0, 0, 0, 1], vec![0, 0, 0, 0, 0], vec![0, 0, 1, 0, 0]]),
            6
        );
    }
}
