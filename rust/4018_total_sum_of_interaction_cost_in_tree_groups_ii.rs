/// LeetCode #4018 - Total Sum of Interaction Cost in Tree Groups II
use std::collections::HashMap;

fn interaction_cost(n: i32, edges: Vec<Vec<i32>>, group: Vec<i32>) -> i64 {
    let n = n as usize;
    let mut g = vec![Vec::new(); n];
    for e in edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        g[u].push(v);
        g[v].push(u);
    }
    let mut tot = vec![0i64; n + 1];
    for &x in &group {
        tot[x as usize] += 1;
    }
    fn dfs(
        u: usize,
        p: usize,
        g: &[Vec<usize>],
        group: &[i32],
        tot: &[i64],
        ans: &mut i64,
    ) -> HashMap<i32, i64> {
        let mut map = HashMap::new();
        map.insert(group[u], 1);
        for &v in &g[u] {
            if v == p {
                continue;
            }
            let mut child = dfs(v, u, g, group, tot, ans);
            for (&grp, &c) in &child {
                *ans += c * (tot[grp as usize] - c);
            }
            if child.len() > map.len() {
                std::mem::swap(&mut map, &mut child);
            }
            for (grp, c) in child {
                *map.entry(grp).or_insert(0) += c;
            }
        }
        map
    }
    let mut ans = 0i64;
    dfs(0, n, &g, &group, &tot, &mut ans);
    ans
}

fn main() {
    println!(
        "{}",
        interaction_cost(3, vec![vec![0, 1], vec![1, 2]], vec![1, 1, 1])
    );
}

#[cfg(test)]
mod tests {
    use super::interaction_cost;

    #[test]
    fn example1() {
        assert_eq!(
            interaction_cost(3, vec![vec![0, 1], vec![1, 2]], vec![1, 1, 1]),
            4
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            interaction_cost(3, vec![vec![0, 1], vec![1, 2]], vec![3, 2, 3]),
            2
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            interaction_cost(
                4,
                vec![vec![0, 1], vec![0, 2], vec![0, 3]],
                vec![1, 1, 4, 4]
            ),
            3
        );
    }

    #[test]
    fn example4() {
        assert_eq!(interaction_cost(2, vec![vec![0, 1]], vec![1, 2]), 0);
    }
}
