/// LeetCode #1666 - Change The Root Of A Binary Tree
#[derive(Clone)]
pub struct Node {
    pub val: i32,
    pub children: Vec<Node>,
}

fn flip_binary_tree(root: Option<Box<Node>>, leaf: i32) -> Option<Box<Node>> {
    fn find_path(node: &Node, leaf: i32, path: &mut Vec<i32>) -> bool {
        path.push(node.val);
        if node.val == leaf { return true; }
        for c in &node.children {
            if find_path(c, leaf, path) { return true; }
        }
        path.pop();
        false
    }
    let root = root?;
    let mut path = vec![];
    if !find_path(root.as_ref(), leaf, &mut path) { return Some(root); }
    let mut cur = root;
    for idx in (1..path.len()).rev() {
        let child_val = path[idx];
        let parent_val = path[idx - 1];
        fn detach(node: &mut Node, child_val: i32, parent_val: i32) {
            if node.val == parent_val {
                node.children.retain(|c| c.val != child_val);
                return;
            }
            for c in &mut node.children { detach(c, child_val, parent_val); }
        }
        detach(cur.as_mut(), child_val, parent_val);
        fn extract(node: &mut Node, val: i32) -> Option<Box<Node>> {
            for i in 0..node.children.len() {
                if node.children[i].val == val {
                    let child = node.children.remove(i);
                    return Some(Box::new(child));
                }
            }
            for c in &mut node.children {
                if let Some(x) = extract(c, val) { return Some(x); }
            }
            None
        }
        if let Some(mut nr) = extract(cur.as_mut(), child_val) {
            nr.children.push(*cur);
            cur = nr;
        }
    }
    Some(cur)
}
fn main() { let _ = flip_binary_tree(None, 0); }
#[cfg(test)]
mod tests {
    use super::{flip_binary_tree, Node};
    #[test]
    fn example_one() {
        let root = Some(Box::new(Node {
            val: 3,
            children: vec![
                Node { val: 5, children: vec![Node { val: 6, children: vec![] }] },
                Node { val: 1, children: vec![Node { val: 2, children: vec![Node { val: 7, children: vec![] }] }] },
            ],
        }));
        assert!(flip_binary_tree(root, 7).is_some());
    }
}