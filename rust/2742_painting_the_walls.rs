/// LeetCode #2742 - Painting the Walls
fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
    let n = cost.len();
    let cols = n * 2 + 1;
    let mut f = vec![vec![-1; cols]; n];
    dfs(&mut f, 0, n as i32, n as i32, &time, &cost)
}

fn dfs(
    f: &mut Vec<Vec<i32>>,
    i: i32,
    j: i32,
    n: i32,
    time: &[i32],
    cost: &[i32],
) -> i32 {
    if n - i <= j - n {
        return 0;
    }
    if i >= n {
        return 1 << 30;
    }
    let ui = i as usize;
    let uj = j as usize;
    if f[ui][uj] == -1 {
        f[ui][uj] = (dfs(f, i + 1, j + time[ui], n, time, cost) + cost[ui])
            .min(dfs(f, i + 1, j - 1, n, time, cost));
    }
    f[ui][uj]
}

fn main() {
    println!("{}", paint_walls(vec![1, 2, 3, 2], vec![1, 2, 3, 2]));
}

#[cfg(test)]
mod tests {
    use super::paint_walls;

    #[test]
    fn example_one() {
        assert_eq!(paint_walls(vec![1, 2, 3, 2], vec![1, 2, 3, 2]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(paint_walls(vec![2, 3, 4, 2], vec![1, 1, 1, 1]), 4);
    }
}
