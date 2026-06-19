/// LeetCode #1971 - Find if Path Exists in Graph
fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    let n = n as usize;
    let mut g = vec![Vec::new(); n];
    for e in edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        g[u].push(v);
        g[v].push(u);
    }

    let mut vis = vec![false; n];
    let mut stk = vec![source as usize];
    while let Some(i) = stk.pop() {
        if i == destination as usize {
            return true;
        }
        if vis[i] {
            continue;
        }
        vis[i] = true;
        for &j in &g[i] {
            if !vis[j] {
                stk.push(j);
            }
        }
    }
    false
}

fn main() {
    println!(
        "{}",
        valid_path(3, vec![vec![0, 1], vec![1, 2], vec![2, 0]], 0, 2)
    );
}

#[cfg(test)]
mod tests {
    use super::valid_path;

    #[test]
    fn example_one() {
        assert!(valid_path(
            3,
            vec![vec![0, 1], vec![1, 2], vec![2, 0]],
            0,
            2
        ));
    }

    #[test]
    fn example_two() {
        assert!(!valid_path(
            6,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![3, 5],
                vec![5, 4],
                vec![4, 3],
            ],
            0,
            5
        ));
    }
}
