/// LeetCode #1273 - Delete Tree Nodes
fn delete_tree_nodes(nodes: Vec<i32>, values: Vec<i32>) -> i32 {
    let n = nodes.len();
    let mut children = vec![vec![]; n];
    for i in 0..n {
        if nodes[i] >= 0 {
            children[nodes[i] as usize].push(i);
        }
    }

    fn dfs(u: usize, children: &[Vec<usize>], values: &[i32]) -> (bool, i32) {
        let mut sub_sum = values[u];
        for &v in &children[u] {
            let (kept, s) = dfs(v, children, values);
            if kept {
                sub_sum += s;
            }
        }
        if sub_sum <= 0 {
            (false, 0)
        } else {
            (true, sub_sum)
        }
    }

    let (kept, sum) = dfs(0, &children, &values);
    if kept { sum } else { 0 }
}

fn main() {
    println!(
        "{}",
        delete_tree_nodes(vec![-1, 0, 0, 1, 1, 0, 0], vec![7, 3, 5, 1, 0, 0, 0])
    );
}

#[cfg(test)]
mod tests {
    use super::delete_tree_nodes;

    #[test]
    fn example_one() {
        assert_eq!(
            delete_tree_nodes(vec![-1, 0, 0, 1, 1, 0, 0], vec![7, 3, 5, 1, 0, 0, 0]),
            6
        );
    }
}
