/// LeetCode #3515 - Shortest Path in a Weighted Tree
struct Fenwick {
    n: usize,
    bit: Vec<i64>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Self {
            n,
            bit: vec![0; n + 2],
        }
    }

    fn add(&mut self, mut i: usize, v: i64) {
        while i <= self.n {
            self.bit[i] += v;
            i += i & i.wrapping_neg();
        }
    }

    fn prefix(&self, mut i: usize) -> i64 {
        let mut s = 0i64;
        while i > 0 {
            s += self.bit[i];
            i -= i & i.wrapping_neg();
        }
        s
    }

    fn range_add(&mut self, l: usize, r: usize, v: i64) {
        self.add(l, v);
        self.add(r + 1, -v);
    }
}

fn tree_queries(n: i32, edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut g = vec![Vec::new(); n + 1];
    let mut weight = std::collections::HashMap::new();
    for e in &edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        let w = e[2] as i64;
        g[u].push((v, w));
        g[v].push((u, w));
        weight.insert((u.min(v), u.max(v)), w);
    }

    let mut tin = vec![0usize; n + 1];
    let mut tout = vec![0usize; n + 1];
    let mut dist = vec![0i64; n + 1];
    let mut parent = vec![0usize; n + 1];
    let mut time = 0usize;

    fn dfs(
        u: usize,
        prev: usize,
        g: &[Vec<(usize, i64)>],
        tin: &mut [usize],
        tout: &mut [usize],
        dist: &mut [i64],
        parent: &mut [usize],
        time: &mut usize,
    ) {
        *time += 1;
        tin[u] = *time;
        for &(v, w) in &g[u] {
            if v == prev {
                continue;
            }
            dist[v] = dist[u] + w;
            parent[v] = u;
            dfs(v, u, g, tin, tout, dist, parent, time);
        }
        tout[u] = *time;
    }

    dfs(1, 0, &g, &mut tin, &mut tout, &mut dist, &mut parent, &mut time);

    let mut fen = Fenwick::new(n);
    for i in 1..=n {
        fen.range_add(tin[i], tin[i], dist[i]);
    }

    let mut ans = Vec::new();
    for q in queries {
        if q[0] == 2 {
            let x = q[1] as usize;
            ans.push(fen.prefix(tin[x]) as i32);
        } else {
            let u = q[1] as usize;
            let v = q[2] as usize;
            let new_w = q[3] as i64;
            let key = (u.min(v), u.max(v));
            let old_w = weight[&key];
            let delta = new_w - old_w;
            weight.insert(key, new_w);
            let child = if parent[v] == u { v } else { u };
            fen.range_add(tin[child], tout[child], delta);
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        tree_queries(2, vec![vec![1, 2, 7]], vec![vec![2, 2], vec![1, 1, 2, 4], vec![2, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::tree_queries;

    #[test]
    fn example1() {
        assert_eq!(
            tree_queries(2, vec![vec![1, 2, 7]], vec![vec![2, 2], vec![1, 1, 2, 4], vec![2, 2]]),
            vec![7, 4]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            tree_queries(
                3,
                vec![vec![1, 2, 2], vec![1, 3, 4]],
                vec![vec![2, 1], vec![2, 3], vec![1, 1, 3, 7], vec![2, 2], vec![2, 3]]
            ),
            vec![0, 4, 2, 7]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            tree_queries(
                4,
                vec![vec![1, 2, 2], vec![2, 3, 1], vec![3, 4, 5]],
                vec![vec![2, 4], vec![2, 3], vec![1, 2, 3, 3], vec![2, 2], vec![2, 3]]
            ),
            vec![8, 3, 2, 5]
        );
    }
}
