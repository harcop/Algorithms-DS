/// LeetCode #1516 - Move Sub Tree Of N Ary Tree
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub val: i32,
    pub children: Vec<Box<Node>>,
}

fn find_node<'a>(root: &'a mut Node, val: i32) -> Option<&'a mut Node> {
    if root.val == val {
        return Some(root);
    }
    for child in root.children.iter_mut() {
        if let Some(found) = find_node(child, val) {
            return Some(found);
        }
    }
    None
}

fn detach(root: &mut Node, val: i32) -> Option<Box<Node>> {
    for i in 0..root.children.len() {
        if root.children[i].val == val {
            return Some(root.children.remove(i));
        }
        if let Some(det) = detach(&mut root.children[i], val) {
            return Some(det);
        }
    }
    None
}

fn move_sub_tree(root: Option<Box<Node>>, p: i32, q: i32) -> Option<Box<Node>> {
    let mut root = root?;
    if p == q {
        return Some(root);
    }
    let mut detached = detach(&mut root, p)?;
    if find_node(&mut root, q).is_some() {
        find_node(&mut root, q).unwrap().children.push(detached);
    }
    Some(root)
}

fn main() {
    let root = Some(Box::new(Node {
        val: 1,
        children: vec![
            Box::new(Node { val: 2, children: vec![Box::new(Node { val: 4, children: vec![] })] }),
            Box::new(Node { val: 3, children: vec![Box::new(Node { val: 5, children: vec![] })] }),
        ],
    }));
    println!("{}", move_sub_tree(root, 4, 5).unwrap().val);
}

#[cfg(test)]
mod tests {
    use super::{move_sub_tree, Node};

    fn build() -> Option<Box<Node>> {
        Some(Box::new(Node {
            val: 1,
            children: vec![
                Box::new(Node { val: 2, children: vec![Box::new(Node { val: 4, children: vec![] })] }),
                Box::new(Node { val: 3, children: vec![Box::new(Node { val: 5, children: vec![] })] }),
            ],
        }))
    }

    #[test]
    fn example_one() {
        let r = move_sub_tree(build(), 4, 5).unwrap();
        assert_eq!(r.val, 1);
        assert_eq!(r.children[1].children[0].children[0].val, 4);
    }
}
