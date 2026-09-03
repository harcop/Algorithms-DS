/// LeetCode #3558 - Number of Ways to Assign Edge Weights I
const MOD: i64 = 1_000_000_007;

fn mod_pow(mut a: i64, mut e: i32) -> i64 {
    let mut r = 1i64;
    a %= MOD;
    while e > 0 {
        if e & 1 == 1 {
            r = r * a % MOD;
        }
        a = a * a % MOD;
        e >>= 1;
    }
    r
}

fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
    let n = edges.len() + 1;
    let mut g = vec![Vec::new(); n + 1];
    for e in &edges {
        let (u, v) = (e[0] as usize, e[1] as usize);
        g[u].push(v);
        g[v].push(u);
    }
    fn dfs(i: usize, fa: usize, g: &[Vec<usize>]) -> i32 {
        let mut res = 0;
        for &j in &g[i] {
            if j != fa {
                res = res.max(dfs(j, i, g) + 1);
            }
        }
        res
    }
    let d = dfs(1, 0, &g);
    mod_pow(2, d - 1) as i32
}

fn main() {
    println!("{}", assign_edge_weights(vec![vec![1, 2]]));
}

#[cfg(test)]
mod tests {
    use super::assign_edge_weights;

    #[test]
    fn example1() {
        assert_eq!(assign_edge_weights(vec![vec![1, 2]]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(
            assign_edge_weights(vec![vec![1, 2], vec![1, 3], vec![3, 4], vec![3, 5]]),
            2
        );
    }
}
