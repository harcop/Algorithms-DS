/// LeetCode #95 - Unique Binary Search Trees II
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn generate_trees(n: i32) -> Vec<Option<Box<TreeNode>>> {
    fn build(lo: i32, hi: i32) -> Vec<Option<Box<TreeNode>>> {
        if lo > hi {
            return vec![None];
        }
        let mut res = Vec::new();
        for i in lo..=hi {
            let lefts = build(lo, i - 1);
            let rights = build(i + 1, hi);
            for l in &lefts {
                for r in &rights {
                    let mut node = TreeNode::new(i);
                    node.left = l.clone();
                    node.right = r.clone();
                    res.push(Some(Box::new(node)));
                }
            }
        }
        res
    }
    build(1, n)
}

fn main() {
    println!("{}", generate_trees(3).len());
}

#[cfg(test)]
mod tests {
    use super::generate_trees;

    #[test]
    fn example_one() {
        assert_eq!(generate_trees(3).len(), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(generate_trees(1).len(), 1);
    }
}
