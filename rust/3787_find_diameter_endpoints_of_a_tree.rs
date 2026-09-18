/// LeetCode #3787 - Find Diameter Endpoints of a Tree (premium)
fn find_special_nodes(n: i32, edges: Vec<Vec<i32>>) -> String {
    let n = n as usize;
    let mut g = vec![Vec::new(); n];
    for e in edges {
        let a = e[0] as usize;
        let b = e[1] as usize;
        g[a].push(b);
        g[b].push(a);
    }
    fn bfs(start: usize, g: &[Vec<usize>]) -> (usize, Vec<i32>) {
        let m = g.len();
        let mut dist = vec![-1i32; m];
        dist[start] = 0;
        let mut q = vec![start];
        let mut far = start;
        let mut i = 0;
        while i < q.len() {
            let u = q[i];
            if dist[u] > dist[far] {
                far = u;
            }
            for &v in &g[u] {
                if dist[v] == -1 {
                    dist[v] = dist[u] + 1;
                    q.push(v);
                }
            }
            i += 1;
        }
        (far, dist)
    }
    let (a, _) = bfs(0, &g);
    let (b, dist1) = bfs(a, &g);
    let (_, dist2) = bfs(b, &g);
    let d = dist1[b];
    let mut ans = vec![b'0'; n];
    for i in 0..n {
        if dist1[i] == d || dist2[i] == d {
            ans[i] = b'1';
        }
    }
    String::from_utf8(ans).unwrap()
}

fn main() {
    println!(
        "{}",
        find_special_nodes(3, vec![vec![0, 1], vec![1, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::find_special_nodes;

    #[test]
    fn example1() {
        assert_eq!(
            find_special_nodes(3, vec![vec![0, 1], vec![1, 2]]),
            "101"
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            find_special_nodes(
                7,
                vec![
                    vec![0, 1],
                    vec![1, 2],
                    vec![2, 3],
                    vec![3, 4],
                    vec![3, 5],
                    vec![1, 6]
                ]
            ),
            "1000111"
        );
    }

    #[test]
    fn example3() {
        assert_eq!(find_special_nodes(2, vec![vec![0, 1]]), "11");
    }
}
