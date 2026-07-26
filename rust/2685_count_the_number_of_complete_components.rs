/// LeetCode #2685 - Count the Number of Complete Components
fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut g = vec![Vec::new(); n];
    for e in &edges {
        let a = e[0] as usize;
        let b = e[1] as usize;
        g[a].push(b);
        g[b].push(a);
    }
    let mut vis = vec![false; n];
    let mut ans = 0;
    for i in 0..n {
        if vis[i] {
            continue;
        }
        let (nodes, edge_ends) = dfs(i, &g, &mut vis);
        if nodes * (nodes - 1) == edge_ends {
            ans += 1;
        }
    }
    ans
}

fn dfs(i: usize, g: &[Vec<usize>], vis: &mut [bool]) -> (i32, i32) {
    vis[i] = true;
    let mut x = 1;
    let mut y = g[i].len() as i32;
    for &j in &g[i] {
        if !vis[j] {
            let (a, b) = dfs(j, g, vis);
            x += a;
            y += b;
        }
    }
    (x, y)
}

fn main() {
    println!(
        "{}",
        count_complete_components(6, vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4]])
    );
}

#[cfg(test)]
mod tests {
    use super::count_complete_components;

    #[test]
    fn example_one() {
        assert_eq!(
            count_complete_components(6, vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4]]),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            count_complete_components(
                6,
                vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4], vec![3, 5]]
            ),
            1
        );
    }
}
