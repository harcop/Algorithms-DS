/// LeetCode #926 - Flip Equivalent Binary Trees
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn flip_equiv(root1: Option<Box<TreeNode>>, root2: Option<Box<TreeNode>>) -> bool {
    fn eq(a: &Option<Box<TreeNode>>, b: &Option<Box<TreeNode>>) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(n1), Some(n2)) => {
                if n1.val != n2.val {
                    return false;
                }
                eq(&n1.left, &n2.left) && eq(&n1.right, &n2.right)
                    || eq(&n1.left, &n2.right) && eq(&n1.right, &n2.left)
            }
            _ => false,
        }
    }
    eq(&root1, &root2)
}

fn main() {
    let t = Some(Box::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    }));
    println!("{}", flip_equiv(t, Some(Box::new(TreeNode { val: 1, left: None, right: None }))));
}

#[cfg(test)]
mod tests {
    use super::{flip_equiv, TreeNode};

    fn node(val: i32, l: Option<Box<TreeNode>>, r: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        Some(Box::new(TreeNode {
            val,
            left: l,
            right: r,
        }))
    }

    #[test]
    fn example_one() {
        let t1 = node(
            1,
            node(
                2,
                node(4, None, None),
                node(5, None, None),
            ),
            node(
                3,
                node(6, None, None),
                node(7, None, None),
            ),
        );
        let t2 = node(
            1,
            node(
                3,
                node(6, None, None),
                node(7, None, None),
            ),
            node(
                2,
                node(4, None, None),
                node(5, None, None),
            ),
        );
        assert!(flip_equiv(t1, t2));
    }
}
