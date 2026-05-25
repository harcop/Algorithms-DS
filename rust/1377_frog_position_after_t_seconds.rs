/// LeetCode #1377 - Frog Position After T Seconds

fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
    let n = n as usize;
    let target = (target - 1) as usize;
    let mut g = vec![vec![]; n];
    for e in edges {
        let u = e[0] as usize - 1;
        let v = e[1] as usize - 1;
        g[u].push(v);
        g[v].push(u);
    }
    let mut visited = vec![false; n];
    fn dfs(
        u: usize,
        parent: usize,
        p: f64,
        time: i32,
        t: i32,
        target: usize,
        g: &[Vec<usize>],
        visited: &mut [bool],
    ) -> f64 {
        if time == t {
            return if u == target { p } else { 0.0 };
        }
        visited[u] = true;
        let children: Vec<usize> = g[u].iter().copied().filter(|&v| v != parent).collect();
        if children.is_empty() {
            return if u == target { p } else { 0.0 };
        }
        let share = p / children.len() as f64;
        let mut sum = 0.0;
        for v in children {
            sum += dfs(v, u, share, time + 1, t, target, g, visited);
        }
        sum
    }
    dfs(0, usize::MAX, 1.0, 0, t, target, &g, &mut visited)
}

fn main() {
    println!("{}", frog_position(7, vec![vec![1, 2], vec![1, 3], vec![1, 7], vec![2, 4], vec![2, 6], vec![3, 5]], 2, 4));
}

#[cfg(test)]
mod tests {
    use super::frog_position;

    #[test]
    fn example_one() {
        let edges = vec![vec![1, 2], vec![1, 3], vec![1, 7], vec![2, 4], vec![2, 6], vec![3, 5]];
        assert!((frog_position(7, edges, 2, 4) - 0.16666666666666666).abs() < 1e-9);
    }
}
