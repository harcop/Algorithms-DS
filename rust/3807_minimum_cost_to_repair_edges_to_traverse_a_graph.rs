/// LeetCode #3807 - Minimum Cost to Repair Edges to Traverse a Graph (premium)
fn min_cost(n: i32, mut edges: Vec<Vec<i32>>, k: i32) -> i32 {
    let n = n as usize;
    let k = k as i32;
    if edges.is_empty() {
        return -1;
    }
    edges.sort_unstable_by_key(|e| e[2]);
    let m = edges.len();
    let check = |idx: usize| -> bool {
        let mut g = vec![Vec::new(); n];
        for e in &edges[..=idx] {
            let u = e[0] as usize;
            let v = e[1] as usize;
            g[u].push(v);
            g[v].push(u);
        }
        let mut q = vec![0];
        let mut vis = vec![false; n];
        vis[0] = true;
        let mut dist = 0;
        while !q.is_empty() {
            let mut nq = Vec::new();
            for &u in &q {
                if u == n - 1 {
                    return dist <= k;
                }
                for &v in &g[u] {
                    if !vis[v] {
                        vis[v] = true;
                        nq.push(v);
                    }
                }
            }
            q = nq;
            dist += 1;
        }
        false
    };
    let mut l = 0;
    let mut r = m - 1;
    while l < r {
        let mid = (l + r) / 2;
        if check(mid) {
            r = mid;
        } else {
            l = mid + 1;
        }
    }
    if check(l) {
        edges[l][2]
    } else {
        -1
    }
}

fn main() {
    println!(
        "{}",
        min_cost(3, vec![vec![0, 1, 10], vec![1, 2, 10], vec![0, 2, 100]], 1)
    );
}

#[cfg(test)]
mod tests {
    use super::min_cost;

    #[test]
    fn example1() {
        assert_eq!(
            min_cost(3, vec![vec![0, 1, 10], vec![1, 2, 10], vec![0, 2, 100]], 1),
            100
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_cost(
                6,
                vec![
                    vec![0, 2, 5],
                    vec![2, 3, 6],
                    vec![3, 4, 7],
                    vec![4, 5, 5],
                    vec![0, 1, 10],
                    vec![1, 5, 12],
                    vec![0, 3, 9],
                    vec![1, 2, 8],
                    vec![2, 4, 11]
                ],
                2
            ),
            12
        );
    }

    #[test]
    fn example3() {
        assert_eq!(min_cost(3, vec![vec![0, 1, 1]], 1), -1);
    }
}
