/// LeetCode #3820 - Pythagorean Distance Nodes in a Tree
fn special_nodes(n: i32, edges: Vec<Vec<i32>>, x: i32, y: i32, z: i32) -> i32 {
    let n = n as usize;
    let mut g = vec![Vec::new(); n];
    for e in &edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        g[u].push(v);
        g[v].push(u);
    }
    let bfs = |src: usize| -> Vec<i32> {
        let mut dist = vec![i32::MAX / 2; n];
        dist[src] = 0;
        let mut q = vec![src];
        let mut i = 0;
        while i < q.len() {
            let u = q[i];
            for &v in &g[u] {
                if dist[v] > dist[u] + 1 {
                    dist[v] = dist[u] + 1;
                    q.push(v);
                }
            }
            i += 1;
        }
        dist
    };
    let d1 = bfs(x as usize);
    let d2 = bfs(y as usize);
    let d3 = bfs(z as usize);
    let mut ans = 0;
    for i in 0..n {
        let mut a = [d1[i] as i64, d2[i] as i64, d3[i] as i64];
        a.sort_unstable();
        if a[0] * a[0] + a[1] * a[1] == a[2] * a[2] {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        special_nodes(4, vec![vec![0, 1], vec![0, 2], vec![0, 3]], 1, 2, 3)
    );
}

#[cfg(test)]
mod tests {
    use super::special_nodes;

    #[test]
    fn example1() {
        assert_eq!(
            special_nodes(4, vec![vec![0, 1], vec![0, 2], vec![0, 3]], 1, 2, 3),
            3
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            special_nodes(4, vec![vec![0, 1], vec![1, 2], vec![2, 3]], 0, 3, 2),
            0
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            special_nodes(4, vec![vec![0, 1], vec![1, 2], vec![1, 3]], 1, 3, 0),
            1
        );
    }
}
