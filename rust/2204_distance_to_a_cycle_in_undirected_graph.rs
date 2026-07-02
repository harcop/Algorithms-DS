/// LeetCode #2204 - Distance to a Cycle in Undirected Graph
use std::collections::{HashSet, VecDeque};

fn distance_to_cycle(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut g: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    for e in edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        g[u].insert(v);
        g[v].insert(u);
    }

    let mut q = VecDeque::new();
    for i in 0..n {
        if g[i].len() == 1 {
            q.push_back(i);
        }
    }

    let mut parent = vec![0usize; n];
    let mut seq = Vec::new();

    while let Some(i) = q.pop_front() {
        seq.push(i);
        for &j in g[i].iter().copied().collect::<Vec<_>>().iter() {
            g[j].remove(&i);
            parent[i] = j;
            if g[j].len() == 1 {
                q.push_back(j);
            }
        }
        g[i].clear();
    }

    let mut ans = vec![0i32; n];
    for &i in seq.iter().rev() {
        ans[i] = ans[parent[i]] + 1;
    }

    ans
}

fn main() {
    println!(
        "{:?}",
        distance_to_cycle(
            7,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 2],
                vec![0, 3],
                vec![0, 4],
                vec![3, 4],
                vec![0, 5],
                vec![0, 6],
            ],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::distance_to_cycle;

    #[test]
    fn example_one() {
        assert_eq!(
            distance_to_cycle(
                7,
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![1, 2],
                    vec![0, 3],
                    vec![0, 4],
                    vec![3, 4],
                    vec![0, 5],
                    vec![0, 6],
                ],
            ),
            vec![0, 0, 0, 0, 0, 1, 1]
        );
    }

    #[test]
    fn leaf_attached_to_cycle() {
        assert_eq!(
            distance_to_cycle(4, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 1]]),
            vec![1, 0, 0, 0]
        );
    }
}
