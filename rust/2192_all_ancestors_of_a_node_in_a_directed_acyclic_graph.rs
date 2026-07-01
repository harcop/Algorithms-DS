/// LeetCode #2192 - All Ancestors of a Node in a Directed Acyclic Graph
use std::collections::VecDeque;

fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut graph = vec![Vec::new(); n];
    let mut indeg = vec![0usize; n];

    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        graph[u].push(v);
        indeg[v] += 1;
    }

    let mut q = VecDeque::new();
    for i in 0..n {
        if indeg[i] == 0 {
            q.push_back(i);
        }
    }

    let mut order = Vec::new();
    while let Some(u) = q.pop_front() {
        order.push(u);
        for &v in &graph[u] {
            indeg[v] -= 1;
            if indeg[v] == 0 {
                q.push_back(v);
            }
        }
    }

    let mut ans = vec![Vec::new(); n];
    for &u in &order {
        let ancestors_u = ans[u].clone();
        for &v in &graph[u] {
            ans[v].extend(ancestors_u.iter().copied());
            ans[v].push(u as i32);
            ans[v].sort_unstable();
            ans[v].dedup();
        }
    }

    ans
}

fn main() {
    println!(
        "{:?}",
        get_ancestors(
            8,
            vec![
                vec![0, 3],
                vec![0, 4],
                vec![1, 3],
                vec![2, 4],
                vec![2, 7],
                vec![3, 5],
                vec![3, 6],
                vec![3, 7],
                vec![4, 6],
            ],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::get_ancestors;

    #[test]
    fn example_one() {
        assert_eq!(
            get_ancestors(
                8,
                vec![
                    vec![0, 3],
                    vec![0, 4],
                    vec![1, 3],
                    vec![2, 4],
                    vec![2, 7],
                    vec![3, 5],
                    vec![3, 6],
                    vec![3, 7],
                    vec![4, 6],
                ],
            ),
            vec![
                vec![],
                vec![],
                vec![],
                vec![0, 1],
                vec![0, 2],
                vec![0, 1, 3],
                vec![0, 1, 2, 3, 4],
                vec![0, 1, 2, 3],
            ]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            get_ancestors(5, vec![]),
            vec![vec![], vec![], vec![], vec![], vec![]]
        );
    }
}
