/// LeetCode #2689 - Extract Kth Character From The Rope Tree
#[derive(Debug)]
struct RopeTreeNode {
    len: i32,
    val: String,
    left: Option<Box<RopeTreeNode>>,
    right: Option<Box<RopeTreeNode>>,
}

impl RopeTreeNode {
    fn leaf(val: &str) -> Self {
        RopeTreeNode {
            len: 0,
            val: val.to_string(),
            left: None,
            right: None,
        }
    }

    fn internal(len: i32, left: RopeTreeNode, right: RopeTreeNode) -> Self {
        RopeTreeNode {
            len,
            val: String::new(),
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
}

fn materialize(root: Option<&RopeTreeNode>) -> String {
    match root {
        None => String::new(),
        Some(node) if node.len == 0 => node.val.clone(),
        Some(node) => {
            materialize(node.left.as_deref()) + &materialize(node.right.as_deref())
        }
    }
}

fn get_kth_character(root: &RopeTreeNode, k: i32) -> char {
    materialize(Some(root)).chars().nth((k - 1) as usize).unwrap()
}

fn main() {
    let root = RopeTreeNode::internal(
        10,
        RopeTreeNode::internal(4, RopeTreeNode::leaf("g"), RopeTreeNode::leaf("rta")),
        RopeTreeNode::leaf("abcpoe"),
    );
    println!("{}", get_kth_character(&root, 6));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let root = RopeTreeNode::internal(
            10,
            RopeTreeNode::internal(4, RopeTreeNode::leaf("g"), RopeTreeNode::leaf("rta")),
            RopeTreeNode::leaf("abcpoe"),
        );
        assert_eq!(get_kth_character(&root, 6), 'b');
    }

    #[test]
    fn example_two() {
        let root = RopeTreeNode::internal(
            12,
            RopeTreeNode::internal(6, RopeTreeNode::leaf("abc"), RopeTreeNode::leaf("efg")),
            RopeTreeNode::internal(6, RopeTreeNode::leaf("hij"), RopeTreeNode::leaf("klm")),
        );
        assert_eq!(get_kth_character(&root, 3), 'c');
    }

    #[test]
    fn example_three() {
        let root = RopeTreeNode::leaf("ropetree");
        assert_eq!(get_kth_character(&root, 8), 'e');
    }
}
