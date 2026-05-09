/// LeetCode #536 - Construct Binary Tree from String
#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn str2tree(s: String) -> Option<Box<TreeNode>> {
    let b = s.as_bytes();
    fn parse(b: &[u8], i: &mut usize) -> Option<Box<TreeNode>> {
        if *i >= b.len() {
            return None;
        }
        let mut sign = 1i32;
        if b[*i] == b'-' {
            sign = -1;
            *i += 1;
        }
        let mut v = 0i32;
        while *i < b.len() && b[*i].is_ascii_digit() {
            v = v * 10 + (b[*i] - b'0') as i32;
            *i += 1;
        }
        v *= sign;
        let mut node = TreeNode {
            val: v,
            left: None,
            right: None,
        };
        if *i < b.len() && b[*i] == b'(' {
            *i += 1;
            node.left = parse(b, i);
            *i += 1;
        }
        if *i < b.len() && b[*i] == b'(' {
            *i += 1;
            node.right = parse(b, i);
            *i += 1;
        }
        Some(Box::new(node))
    }
    let mut i = 0usize;
    parse(b, &mut i)
}

fn main() {
    println!("{:?}", str2tree("4(2(3)(1))(6(5))".into()).map(|n| n.val));
}

#[cfg(test)]
mod tests {
    use super::str2tree;

    #[test]
    fn example_one() {
        let r = str2tree("4(2(3)(1))(6(5))".into()).unwrap();
        assert_eq!(r.val, 4);
        assert_eq!(r.left.as_ref().unwrap().val, 2);
        assert_eq!(r.right.as_ref().unwrap().val, 6);
    }
}
