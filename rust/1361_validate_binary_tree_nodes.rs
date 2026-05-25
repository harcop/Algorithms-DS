/// LeetCode #1361 - Validate Binary Tree Nodes

fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
    let n = n as usize;
    let mut indegree = vec![0usize; n];
    for i in 0..n {
        for c in [left_child[i], right_child[i]] {
            if c != -1 {
                let c = c as usize;
                indegree[c] += 1;
                if indegree[c] > 1 {
                    return false;
                }
            }
        }
    }
    let roots: Vec<usize> = (0..n).filter(|&i| indegree[i] == 0).collect();
    if roots.len() != 1 {
        return false;
    }
    let mut seen = 0usize;
    let mut q = vec![roots[0]];
    while let Some(u) = q.pop() {
        seen += 1;
        for c in [left_child[u], right_child[u]] {
            if c != -1 {
                let c = c as usize;
                indegree[c] -= 1;
                if indegree[c] == 0 {
                    q.push(c);
                }
            }
        }
    }
    seen == n
}

fn main() {
    println!("{}", validate_binary_tree_nodes(4, vec![-1, -1, 0, -1], vec![-1, -1, -1, 1]));
}

#[cfg(test)]
mod tests {
    use super::validate_binary_tree_nodes;

    #[test]
    fn example_one() {
        assert!(validate_binary_tree_nodes(3, vec![1, -1, -1], vec![-1, 2, -1]));
    }

    #[test]
    fn example_two() {
        assert!(!validate_binary_tree_nodes(4, vec![1, -1, 0, -1], vec![2, -1, -1, 1]));
    }
}
