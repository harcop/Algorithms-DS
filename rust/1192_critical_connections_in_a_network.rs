/// LeetCode #1192 - Critical Connections in a Network
fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    for e in &connections {
        let u = e[0] as usize;
        let v = e[1] as usize;
        g[u].push(v);
        g[v].push(u);
    }
    let mut disc = vec![0i32; n];
    let mut low = vec![0i32; n];
    let mut time = 0i32;
    let mut bridges = Vec::new();
    fn dfs(
        u: usize,
        p: usize,
        g: &[Vec<usize>],
        disc: &mut [i32],
        low: &mut [i32],
        time: &mut i32,
        bridges: &mut Vec<Vec<i32>>,
    ) {
        *time += 1;
        disc[u] = *time;
        low[u] = *time;
        for &v in &g[u] {
            if v == p {
                continue;
            }
            if disc[v] == 0 {
                dfs(v, u, g, disc, low, time, bridges);
                low[u] = low[u].min(low[v]);
                if low[v] > disc[u] {
                    bridges.push(vec![u as i32, v as i32]);
                }
            } else {
                low[u] = low[u].min(disc[v]);
            }
        }
    }
    dfs(0, usize::MAX, &g, &mut disc, &mut low, &mut time, &mut bridges);
    bridges
}

fn main() {
    println!(
        "{:?}",
        critical_connections(4, vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![1, 3]])
    );
}

#[cfg(test)]
mod tests {
    use super::critical_connections;

    fn mut_sort(v: &mut Vec<Vec<i32>>) {
        for e in v.iter_mut() {
            e.sort();
        }
        v.sort_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));
    }

    #[test]
    fn example_one() {
        let mut got = critical_connections(4, vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![1, 3]]);
        mut_sort(&mut got);
        assert_eq!(got, vec![vec![1, 3]]);
    }
}
