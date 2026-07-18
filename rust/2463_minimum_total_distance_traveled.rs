/// LeetCode #2463 - Minimum Total Distance Traveled
fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<Vec<i32>>) -> i64 {
    robot.sort_unstable();
    factory.sort_unstable_by_key(|f| f[0]);

    let n = robot.len();
    let m = factory.len();
    let mut memo = vec![vec![None; m + 1]; n + 1];

    fn dfs(
        i: usize,
        j: usize,
        robot: &[i32],
        factory: &[Vec<i32>],
        memo: &mut [Vec<Option<i64>>],
    ) -> i64 {
        if i == robot.len() {
            return 0;
        }
        if j == factory.len() {
            return i64::MAX / 4;
        }
        if let Some(cached) = memo[i][j] {
            return cached;
        }

        let mut best = dfs(i, j + 1, robot, factory, memo);
        let position = factory[j][0] as i64;
        let limit = factory[j][1] as usize;
        let mut total = 0i64;

        for k in 0..limit {
            if i + k >= robot.len() {
                break;
            }
            total += (robot[i + k] as i64 - position).abs();
            best = best.min(total + dfs(i + k + 1, j + 1, robot, factory, memo));
        }

        memo[i][j] = Some(best);
        best
    }

    dfs(0, 0, &robot, &factory, &mut memo)
}

fn main() {
    println!(
        "{}",
        minimum_total_distance(vec![0, 4, 6], vec![vec![2, 2], vec![6, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_total_distance;

    #[test]
    fn example_one() {
        assert_eq!(
            minimum_total_distance(vec![0, 4, 6], vec![vec![2, 2], vec![6, 2]]),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            minimum_total_distance(vec![1, -1], vec![vec![-2, 1], vec![2, 1]]),
            2
        );
    }
}
