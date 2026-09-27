/// LeetCode #3973 - Distinct Gate Paths to LCA
fn distinct_gate_paths(n: i32, parent: Vec<i32>, gates: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    const LOG: usize = 15;
    let n = n as usize;
    let mut children = vec![vec![]; n];
    for i in 1..n {
        children[parent[i] as usize].push(i);
    }
    let mut depth = vec![0usize; n];
    let mut stack = vec![0usize];
    while let Some(u) = stack.pop() {
        for &v in &children[u] {
            depth[v] = depth[u] + 1;
            stack.push(v);
        }
    }

    let mat_of = |g: &[i32]| -> [[i64; 2]; 2] {
        let r = g[0] as i64;
        let b = g[1] as i64;
        let w = g[2] as i64;
        [[r, w], [w, b]]
    };
    let mul = |a: [[i64; 2]; 2], b: [[i64; 2]; 2]| -> [[i64; 2]; 2] {
        let mut c = [[0i64; 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                c[i][j] = (a[i][0] * b[0][j] + a[i][1] * b[1][j]) % MOD;
            }
        }
        c
    };

    let mut up = vec![vec![0usize; n]; LOG];
    let mut mat = vec![vec![[[1i64, 0], [0, 1]]; n]; LOG];
    for u in 0..n {
        up[0][u] = if u == 0 { 0 } else { parent[u] as usize };
        mat[0][u] = mat_of(&gates[u]);
    }
    for k in 1..LOG {
        for u in 0..n {
            let mid = up[k - 1][u];
            up[k][u] = up[k - 1][mid];
            mat[k][u] = mul(mat[k - 1][mid], mat[k - 1][u]);
        }
    }

    let lca = |mut a: usize, mut b: usize| -> usize {
        if depth[a] < depth[b] {
            std::mem::swap(&mut a, &mut b);
        }
        let mut diff = depth[a] - depth[b];
        for k in (0..LOG).rev() {
            if diff >= (1 << k) {
                a = up[k][a];
                diff -= 1 << k;
            }
        }
        if a == b {
            return a;
        }
        for k in (0..LOG).rev() {
            if up[k][a] != up[k][b] {
                a = up[k][a];
                b = up[k][b];
            }
        }
        up[0][a]
    };

    let ways = |mut node: usize, card: i32, anc: usize| -> i64 {
        if node == anc {
            return 1;
        }
        let mut steps = depth[node] - depth[anc];
        let mut acc = [[1i64, 0], [0, 1]];
        for k in 0..LOG {
            if steps & (1 << k) != 0 {
                acc = mul(mat[k][node], acc);
                node = up[k][node];
                steps -= 1 << k;
            }
        }
        let (red, blue) = if card == 1 { (1i64, 0i64) } else { (0i64, 1i64) };
        let nr = (acc[0][0] * red + acc[0][1] * blue) % MOD;
        let nb = (acc[1][0] * red + acc[1][1] * blue) % MOD;
        (nr + nb) % MOD
    };

    let mut ans = 0i32;
    for q in queries {
        let anc = lca(q[0] as usize, q[2] as usize);
        let wa = ways(q[0] as usize, q[1], anc);
        let wb = ways(q[2] as usize, q[3], anc);
        ans ^= ((wa * wb) % MOD) as i32;
    }
    ans
}

fn main() {
    println!(
        "{}",
        distinct_gate_paths(
            3,
            vec![-1, 0, 0],
            vec![vec![1, 0, 1], vec![0, 1, 1], vec![1, 1, 0]],
            vec![vec![1, 0, 2, 0], vec![1, 1, 2, 0], vec![1, 0, 2, 1]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::distinct_gate_paths;

    #[test]
    fn example1() {
        assert_eq!(
            distinct_gate_paths(
                3,
                vec![-1, 0, 0],
                vec![vec![1, 0, 1], vec![0, 1, 1], vec![1, 1, 0]],
                vec![vec![1, 0, 2, 0], vec![1, 1, 2, 0], vec![1, 0, 2, 1]]
            ),
            1
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            distinct_gate_paths(
                3,
                vec![-1, 0, 1],
                vec![vec![0, 1, 2], vec![1, 0, 1], vec![0, 0, 3]],
                vec![vec![2, 0, 1, 0], vec![2, 1, 0, 0], vec![1, 1, 2, 1]]
            ),
            3
        );
    }
}
