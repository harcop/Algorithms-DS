/// LeetCode #865 - Smallest Subtree with all the Deepest Nodes
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<TreeNode>>,
    pub right: Option<Rc<TreeNode>>,
}

fn subtree_with_all_deepest(root: Option<Rc<TreeNode>>) -> Option<Rc<TreeNode>> {
    fn dfs(node: Option<Rc<TreeNode>>) -> (i32, Option<Rc<TreeNode>>) {
        match node {
            None => (-1, None),
            Some(n) => {
                let (dl, left) = dfs(n.left.clone());
                let (dr, right) = dfs(n.right.clone());
                if dl == dr {
                    (dl + 1, Some(n))
                } else if dl > dr {
                    (dl + 1, left)
                } else {
                    (dr + 1, right)
                }
            }
        }
    }
    dfs(root).1
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::{subtree_with_all_deepest, TreeNode};
    use std::rc::Rc;

    fn node(val: i32, left: Option<Rc<TreeNode>>, right: Option<Rc<TreeNode>>) -> Rc<TreeNode> {
        Rc::new(TreeNode { val, left, right })
    }

    #[test]
    fn example_one() {
        let two = node(
            2,
            Some(node(7, None, None)),
            Some(node(4, None, None)),
        );
        let five = node(
            5,
            Some(node(6, None, None)),
            Some(two),
        );
        let root = node(3, Some(five), Some(node(1, None, None)));
        let ans = subtree_with_all_deepest(Some(root));
        assert_eq!(ans.unwrap().val, 2);
    }
}
