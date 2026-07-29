/// LeetCode #2764 - Is Array a Preorder of Some Binary Tree
use std::collections::HashMap;

fn is_preorder(nodes: Vec<Vec<i32>>) -> bool {
    let mut g: HashMap<i32, Vec<i32>> = HashMap::new();
    for node in &nodes {
        g.entry(node[1]).or_default().push(node[0]);
    }
    let mut k = 0usize;
    fn dfs(i: i32, nodes: &[Vec<i32>], k: &mut usize, g: &HashMap<i32, Vec<i32>>) -> bool {
        if i != nodes[*k][0] {
            return false;
        }
        *k += 1;
        if let Some(children) = g.get(&i) {
            for &j in children {
                if !dfs(j, nodes, k, g) {
                    return false;
                }
            }
        }
        true
    }
    let root = nodes[0][0];
    dfs(root, &nodes, &mut k, &g) && k == nodes.len()
}

fn main() {
    println!(
        "{}",
        is_preorder(vec![vec![1, -1], vec![2, 1], vec![3, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::is_preorder;

    #[test]
    fn example_one() {
        assert!(is_preorder(vec![vec![1, -1], vec![2, 1], vec![3, 1]]));
    }

    #[test]
    fn example_two() {
        // root=1, children=[2,3], child of 2=[4]. Preorder: 1,2,4,3.
        // Input [1,2,3,4] visits 3 before 4 — not valid preorder.
        assert!(!is_preorder(vec![
            vec![1, -1],
            vec![2, 1],
            vec![3, 1],
            vec![4, 2]
        ]));
    }
}
