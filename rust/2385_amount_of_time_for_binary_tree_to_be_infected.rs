/// LeetCode #2385 - Amount of Time for Binary Tree to Be Infected
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
    let mut g: HashMap<i32, Vec<i32>> = HashMap::new();

    fn build(node: &Option<Rc<RefCell<TreeNode>>>, fa: Option<i32>, g: &mut HashMap<i32, Vec<i32>>) {
        if let Some(n) = node {
            let n = n.borrow();
            if let Some(parent) = fa {
                g.entry(n.val).or_default().push(parent);
                g.entry(parent).or_default().push(n.val);
            }
            build(&n.left, Some(n.val), g);
            build(&n.right, Some(n.val), g);
        }
    }

    fn dfs(node: i32, fa: i32, g: &HashMap<i32, Vec<i32>>) -> i32 {
        let mut ans = 0;
        if let Some(neighbors) = g.get(&node) {
            for &nxt in neighbors {
                if nxt != fa {
                    ans = ans.max(1 + dfs(nxt, node, g));
                }
            }
        }
        ans
    }

    build(&root, None, &mut g);
    dfs(start, -1, &g)
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    println!("{}", amount_of_time(root, 1));
}

#[cfg(test)]
mod tests {
    use super::{amount_of_time, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    fn node(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode::new(val)))
    }

    #[test]
    fn example_one() {
        // [1,5,3,null,4,10,6,9,2], start = 3
        let n9 = node(9);
        let n2 = node(2);
        let n4 = node(4);
        n4.borrow_mut().left = Some(n9);
        n4.borrow_mut().right = Some(n2);
        let n5 = node(5);
        n5.borrow_mut().right = Some(n4);
        let n10 = node(10);
        let n6 = node(6);
        let n3 = node(3);
        n3.borrow_mut().left = Some(n10);
        n3.borrow_mut().right = Some(n6);
        let root = node(1);
        root.borrow_mut().left = Some(n5);
        root.borrow_mut().right = Some(n3);
        assert_eq!(amount_of_time(Some(root), 3), 4);
    }

    #[test]
    fn example_two() {
        let root = Some(node(1));
        assert_eq!(amount_of_time(root, 1), 0);
    }
}
