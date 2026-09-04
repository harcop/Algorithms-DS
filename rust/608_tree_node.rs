/// LeetCode #608 - Tree Node (SQL; Rust analogue)
use std::collections::HashSet;

fn tree_node(tree: Vec<(i32, Option<i32>)>) -> Vec<(i32, String)> {
    let parents: HashSet<i32> = tree.iter().filter_map(|(_, p)| *p).collect();
    let mut ans = Vec::new();
    for (id, p) in tree {
        let typ = if p.is_none() {
            "Root"
        } else if parents.contains(&id) {
            "Inner"
        } else {
            "Leaf"
        };
        ans.push((id, typ.to_string()));
    }
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::tree_node;

    #[test]
    fn example() {
        let tree = vec![
            (1, None),
            (2, Some(1)),
            (3, Some(1)),
            (4, Some(2)),
            (5, Some(2)),
        ];
        assert_eq!(
            tree_node(tree),
            vec![
                (1, "Root".into()),
                (2, "Inner".into()),
                (3, "Leaf".into()),
                (4, "Leaf".into()),
                (5, "Leaf".into()),
            ]
        );
    }
}
