/// LeetCode #1379 - Find A Corresponding Node Of A Binary Tree In A Clone Of That Tree
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn get_target_copy(
    original: Option<Box<TreeNode>>,
    cloned: Option<Box<TreeNode>>,
    target: &TreeNode,
) -> Option<Box<TreeNode>> {
    fn walk(
        o: &Option<Box<TreeNode>>,
        c: &Option<Box<TreeNode>>,
        target_val: i32,
    ) -> Option<Box<TreeNode>> {
        let (on, cn) = (o.as_ref()?, c.as_ref()?);
        if on.val == target_val {
            return Some(Box::new(TreeNode {
                val: cn.val,
                left: None,
                right: None,
            }));
        }
        walk(&on.left, &cn.left, target_val).or_else(|| walk(&on.right, &cn.right, target_val))
    }
    walk(&original, &cloned, target.val)
}

fn main() {
    let mut o = Box::new(TreeNode { val: 7, left: None, right: None });
    o.left = Some(Box::new(TreeNode { val: 4, left: None, right: None }));
    let target_val = o.left.as_ref().unwrap().val;
    let c = o.clone();
    println!("{:?}", get_target_copy(Some(o), Some(c), &TreeNode { val: target_val, left: None, right: None }).map(|n| n.val));
}

#[cfg(test)]
mod tests {
    use super::{get_target_copy, TreeNode};

    #[test]
    fn example_one() {
        let mut o = Box::new(TreeNode { val: 7, left: None, right: None });
        o.left = Some(Box::new(TreeNode { val: 4, left: None, right: None }));
        o.right = Some(Box::new(TreeNode { val: 3, left: None, right: None }));
        let target = TreeNode { val: 4, left: None, right: None };
        let c = o.clone();
        assert_eq!(get_target_copy(Some(o), Some(c), &target).unwrap().val, 4);
    }
}
