/// LeetCode #2196 - Create Binary Tree From Descriptions
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Box<TreeNode>> {
    let mut left_child = HashMap::new();
    let mut right_child = HashMap::new();
    let mut children = HashSet::new();
    let mut nodes = HashSet::new();

    for desc in descriptions {
        let parent = desc[0];
        let child = desc[1];
        nodes.insert(parent);
        nodes.insert(child);
        children.insert(child);
        if desc[2] == 1 {
            left_child.insert(parent, child);
        } else {
            right_child.insert(parent, child);
        }
    }

    let root = nodes.iter().find(|v| !children.contains(v))?;

    fn build(
        val: i32,
        left_child: &HashMap<i32, i32>,
        right_child: &HashMap<i32, i32>,
    ) -> Box<TreeNode> {
        Box::new(TreeNode {
            val,
            left: left_child
                .get(&val)
                .map(|&c| build(c, left_child, right_child)),
            right: right_child
                .get(&val)
                .map(|&c| build(c, left_child, right_child)),
        })
    }

    Some(build(*root, &left_child, &right_child))
}

fn main() {
    let root = create_binary_tree(vec![
        vec![20, 15, 1],
        vec![20, 17, 0],
        vec![50, 20, 1],
        vec![50, 80, 0],
        vec![80, 19, 1],
    ]);
    println!("{:?}", root.as_ref().map(|n| n.val));
}

#[cfg(test)]
mod tests {
    use super::create_binary_tree;

    #[test]
    fn example_one() {
        let root = create_binary_tree(vec![
            vec![20, 15, 1],
            vec![20, 17, 0],
            vec![50, 20, 1],
            vec![50, 80, 0],
            vec![80, 19, 1],
        ])
        .unwrap();
        assert_eq!(root.val, 50);
        assert_eq!(root.left.as_ref().unwrap().val, 20);
        assert_eq!(root.right.as_ref().unwrap().val, 80);
        assert_eq!(root.left.as_ref().unwrap().left.as_ref().unwrap().val, 15);
        assert_eq!(root.left.as_ref().unwrap().right.as_ref().unwrap().val, 17);
        assert_eq!(root.right.as_ref().unwrap().left.as_ref().unwrap().val, 19);
    }

    #[test]
    fn example_two() {
        let root = create_binary_tree(vec![vec![1, 2, 1], vec![2, 3, 0], vec![3, 4, 1]]).unwrap();
        assert_eq!(root.val, 1);
        assert_eq!(root.left.as_ref().unwrap().val, 2);
        assert_eq!(root.left.is_some(), true);
        assert_eq!(root.right.is_none(), true);
        assert_eq!(
            root.left
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .val,
            4
        );
    }
}
