/// LeetCode #2242 - Maximum Score of a Node Sequence
use std::collections::HashMap;

fn maximum_score(scores: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    for e in &edges {
        let a = e[0];
        let b = e[1];
        graph.entry(a).or_default().push(b);
        graph.entry(b).or_default().push(a);
    }

    for neighbors in graph.values_mut() {
        neighbors.sort_unstable_by_key(|&x| std::cmp::Reverse(scores[x as usize]));
        neighbors.truncate(3);
    }

    let mut ans = -1i32;
    for e in edges {
        let u = e[0];
        let b = e[1];
        let Some(neighbors_u) = graph.get(&u) else {
            continue;
        };
        let Some(neighbors_b) = graph.get(&b) else {
            continue;
        };
        for &a in neighbors_u {
            for &d in neighbors_b {
                if a != d && a != b && d != u {
                    let total = scores[a as usize]
                        + scores[u as usize]
                        + scores[b as usize]
                        + scores[d as usize];
                    ans = ans.max(total);
                }
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        maximum_score(
            vec![5, 2, 9, 8, 6],
            vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![2, 3], vec![2, 4]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_score;

    #[test]
    fn example_one() {
        assert_eq!(
            maximum_score(
                vec![5, 2, 9, 8, 6],
                vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![2, 3], vec![2, 4]]
            ),
            24
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_score(vec![1, 1, 1], vec![vec![0, 1], vec![1, 2]]), -1);
    }
}
