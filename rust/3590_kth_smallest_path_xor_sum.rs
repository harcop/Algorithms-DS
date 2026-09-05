/// LeetCode #3590 - Kth Smallest Path XOR Sum
use std::collections::BTreeSet;

fn kth_smallest(par: Vec<i32>, vals: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = par.len();
    let mut g = vec![Vec::new(); n];
    for i in 1..n {
        g[par[i] as usize].push(i);
    }
    let mut path_xor = vals.clone();
    fn compute_xor(u: usize, acc: i32, g: &[Vec<usize>], path_xor: &mut [i32]) {
        path_xor[u] ^= acc;
        let cur = path_xor[u];
        for &v in &g[u] {
            compute_xor(v, cur, g, path_xor);
        }
    }
    compute_xor(0, 0, &g, &mut path_xor);
    let mut by_node = vec![Vec::new(); n];
    for (idx, q) in queries.iter().enumerate() {
        by_node[q[0] as usize].push((q[1], idx));
    }
    let mut ans = vec![0; queries.len()];
    fn dfs(
        u: usize,
        g: &[Vec<usize>],
        path_xor: &[i32],
        by_node: &[Vec<(i32, usize)>],
        ans: &mut [i32],
    ) -> BTreeSet<i32> {
        let mut set = BTreeSet::new();
        set.insert(path_xor[u]);
        for &v in &g[u] {
            let mut cs = dfs(v, g, path_xor, by_node, ans);
            if cs.len() > set.len() {
                std::mem::swap(&mut set, &mut cs);
            }
            set.extend(cs);
        }
        for &(k, idx) in &by_node[u] {
            ans[idx] = set.iter().nth((k as usize).saturating_sub(1)).copied().unwrap_or(-1);
        }
        set
    }
    dfs(0, &g, &path_xor, &by_node, &mut ans);
    ans
}

fn main() {
    println!("{:?}", kth_smallest(vec![-1, 0, 0], vec![1, 1, 1], vec![vec![0, 1], vec![0, 2], vec![0, 3]]));
}

#[cfg(test)]
mod tests {
    use super::kth_smallest;

    #[test]
    fn example1() {
        assert_eq!(
            kth_smallest(vec![-1, 0, 0], vec![1, 1, 1], vec![vec![0, 1], vec![0, 2], vec![0, 3]]),
            vec![0, 1, -1]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            kth_smallest(
                vec![-1, 0, 1],
                vec![5, 2, 7],
                vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![2, 1]]
            ),
            vec![0, 7, -1, 0]
        );
    }
}
