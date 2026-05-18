/// LeetCode #1066 - Campus Bikes II
fn assign_bikes(workers: Vec<Vec<i32>>, bikes: Vec<Vec<i32>>) -> i32 {
    let n = workers.len();
    let m = bikes.len();
    let mut dist = vec![vec![0i32; m]; n];
    for i in 0..n {
        for j in 0..m {
            dist[i][j] = (workers[i][0] - bikes[j][0]).abs() + (workers[i][1] - bikes[j][1]).abs();
        }
    }
    let mut memo = vec![vec![-1i32; 1 << m]; n + 1];
    fn dfs(i: usize, mask: usize, dist: &Vec<Vec<i32>>, memo: &mut Vec<Vec<i32>>, m: usize) -> i32 {
        if i == dist.len() {
            return 0;
        }
        if memo[i][mask] != -1 {
            return memo[i][mask];
        }
        let mut best = i32::MAX;
        for j in 0..m {
            if mask & (1 << j) == 0 {
                let v = dist[i][j] + dfs(i + 1, mask | (1 << j), dist, memo, m);
                best = best.min(v);
            }
        }
        memo[i][mask] = best;
        best
    }
    dfs(0, 0, &dist, &mut memo, m)
}

fn main() {
    println!("{}", assign_bikes(vec![vec![0, 0], vec![2, 1]], vec![vec![1, 2], vec![3, 3]]));
}

#[cfg(test)]
mod tests {
    use super::assign_bikes;

    #[test]
    fn example_one() {
        assert_eq!(assign_bikes(vec![vec![0, 0], vec![2, 1]], vec![vec![1, 2], vec![3, 3]]), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(assign_bikes(vec![vec![0, 0], vec![1, 1], vec![2, 0]], vec![vec![1, 0], vec![2, 2], vec![2, 1]]), 4);
    }
}
