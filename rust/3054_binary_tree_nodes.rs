/// LeetCode #3054 - Binary Tree Nodes (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn binary_tree_nodes(nodes: Vec<(i32, Option<i32>)>) -> Vec<(i32, String)> {
    let mut parent_of: HashMap<i32, Option<i32>> = HashMap::new();
    let mut children: HashSet<i32> = HashSet::new();

    for (n, p) in &nodes {
        parent_of.insert(*n, *p);
        if let Some(parent) = p {
            children.insert(*parent);
        }
    }

    let mut ans: Vec<_> = nodes
        .into_iter()
        .map(|(n, p)| {
            let node_type = if p.is_none() {
                "Root"
            } else if !children.contains(&n) {
                "Leaf"
            } else {
                "Inner"
            };
            (n, node_type.to_string())
        })
        .collect();
    ans.sort_unstable_by_key(|(n, _)| *n);
    ans
}

fn main() {
    let nodes = vec![
        (1, Some(2)),
        (3, Some(2)),
        (6, Some(8)),
        (9, Some(8)),
        (2, Some(5)),
        (8, Some(5)),
        (5, None),
    ];
    println!("{:?}", binary_tree_nodes(nodes));
}

#[cfg(test)]
mod tests {
    use super::binary_tree_nodes;

    #[test]
    fn example() {
        let nodes = vec![
            (1, Some(2)),
            (3, Some(2)),
            (6, Some(8)),
            (9, Some(8)),
            (2, Some(5)),
            (8, Some(5)),
            (5, None),
        ];
        assert_eq!(
            binary_tree_nodes(nodes),
            vec![
                (1, "Leaf".into()),
                (2, "Inner".into()),
                (3, "Leaf".into()),
                (5, "Root".into()),
                (6, "Leaf".into()),
                (8, "Inner".into()),
                (9, "Leaf".into()),
            ]
        );
    }
}
