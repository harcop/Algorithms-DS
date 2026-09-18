/// LeetCode #3786 - Total Sum of Interaction Cost in Tree Groups
fn interaction_costs(n: i32, edges: Vec<Vec<i32>>, group: Vec<i32>) -> i64 {
    let n = n as usize;
    let mut adj = vec![Vec::new(); n];
    for e in &edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut total = [0i64; 21];
    for &g in &group {
        total[g as usize] += 1;
    }
    let mut parent = vec![usize::MAX; n];
    let mut order = vec![0];
    parent[0] = 0;
    let mut i = 0;
    while i < order.len() {
        let u = order[i];
        for &v in &adj[u] {
            if v == parent[u] {
                continue;
            }
            parent[v] = u;
            order.push(v);
        }
        i += 1;
    }
    parent[0] = usize::MAX;
    let mut sub = vec![[0i64; 21]; n];
    let mut ans = 0i64;
    for &u in order.iter().rev() {
        sub[u][group[u] as usize] = 1;
        for &v in &adj[u] {
            if v == parent[u] {
                continue;
            }
            for g in 1..21 {
                ans += sub[v][g] * (total[g] - sub[v][g]);
                sub[u][g] += sub[v][g];
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        interaction_costs(3, vec![vec![0, 1], vec![1, 2]], vec![1, 1, 1])
    );
}

#[cfg(test)]
mod tests {
    use super::interaction_costs;

    #[test]
    fn example1() {
        assert_eq!(
            interaction_costs(3, vec![vec![0, 1], vec![1, 2]], vec![1, 1, 1]),
            4
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            interaction_costs(3, vec![vec![0, 1], vec![1, 2]], vec![3, 2, 3]),
            2
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            interaction_costs(
                4,
                vec![vec![0, 1], vec![0, 2], vec![0, 3]],
                vec![1, 1, 4, 4]
            ),
            3
        );
    }

    #[test]
    fn example4() {
        assert_eq!(interaction_costs(2, vec![vec![0, 1]], vec![9, 8]), 0);
    }
}
