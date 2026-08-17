/// LeetCode #3241 - Time Taken to Mark All Nodes
fn time_taken(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = edges.len() + 1;
    let mut g = vec![vec![]; n];
    for e in &edges {
        let a = e[0] as usize;
        let b = e[1] as usize;
        g[a].push(b);
        g[b].push(a);
    }
    let cost = |u: usize| -> i32 {
        if u % 2 == 1 {
            1
        } else {
            2
        }
    };
    let mut down = vec![0i32; n];
    let mut best = vec![0i32; n];
    let mut second = vec![0i32; n];
    let mut best_child = vec![usize::MAX; n];

    fn dfs1(
        g: &[Vec<usize>],
        u: usize,
        fa: usize,
        cost: &dyn Fn(usize) -> i32,
        down: &mut [i32],
        best: &mut [i32],
        second: &mut [i32],
        best_child: &mut [usize],
    ) {
        for &v in &g[u] {
            if v == fa {
                continue;
            }
            dfs1(g, v, u, cost, down, best, second, best_child);
            let t = down[v] + cost(v);
            if t > best[u] {
                second[u] = best[u];
                best[u] = t;
                best_child[u] = v;
            } else if t > second[u] {
                second[u] = t;
            }
        }
        down[u] = best[u];
    }
    dfs1(
        &g,
        0,
        usize::MAX,
        &cost,
        &mut down,
        &mut best,
        &mut second,
        &mut best_child,
    );

    let mut ans = vec![0i32; n];
    let mut up = vec![0i32; n];
    fn dfs2(
        g: &[Vec<usize>],
        u: usize,
        fa: usize,
        cost: &dyn Fn(usize) -> i32,
        down: &[i32],
        best: &[i32],
        second: &[i32],
        best_child: &[usize],
        up: &mut [i32],
        ans: &mut [i32],
    ) {
        ans[u] = down[u].max(up[u]);
        for &v in &g[u] {
            if v == fa {
                continue;
            }
            let cand = if best_child[u] == v {
                second[u]
            } else {
                best[u]
            };
            up[v] = cost(u) + up[u].max(cand);
            dfs2(
                g, v, u, cost, down, best, second, best_child, up, ans,
            );
        }
    }
    dfs2(
        &g,
        0,
        usize::MAX,
        &cost,
        &down,
        &best,
        &second,
        &best_child,
        &mut up,
        &mut ans,
    );
    ans
}

fn main() {
    println!("{:?}", time_taken(vec![vec![0, 1], vec![0, 2]]));
}

#[cfg(test)]
mod tests {
    use super::time_taken;

    #[test]
    fn example1() {
        assert_eq!(time_taken(vec![vec![0, 1], vec![0, 2]]), vec![2, 4, 3]);
    }

    #[test]
    fn example2() {
        assert_eq!(time_taken(vec![vec![0, 1]]), vec![1, 2]);
    }

    #[test]
    fn example3() {
        assert_eq!(
            time_taken(vec![vec![2, 4], vec![0, 1], vec![2, 3], vec![0, 2]]),
            vec![4, 6, 3, 5, 5]
        );
    }
}
