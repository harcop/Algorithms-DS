/// LeetCode #1522 - Diameter Of N Ary Tree
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub val: i32,
    pub children: Vec<Box<Node>>,
}

fn diameter(root: Option<Box<Node>>) -> i32 {
    fn dfs(node: &Node) -> (i32, i32) {
        let mut best = 0;
        let mut top1 = 0;
        let mut top2 = 0;
        for c in &node.children {
            let (diam, h) = dfs(c);
            best = best.max(diam);
            if h > top1 {
                top2 = top1;
                top1 = h;
            } else if h > top2 {
                top2 = h;
            }
        }
        best = best.max(top1 + top2);
        (best, top1 + 1)
    }
    root.map(|r| dfs(&r).0).unwrap_or(0)
}

fn main() {
    let root = Some(Box::new(Node {
        val: 1,
        children: vec![
            Box::new(Node { val: 2, children: vec![Box::new(Node { val: 3, children: vec![] })] }),
            Box::new(Node { val: 4, children: vec![] }),
        ],
    }));
    println!("{}", diameter(root));
}

#[cfg(test)]
mod tests {
    use super::{diameter, Node};

    #[test]
    fn example_one() {
        let root = Some(Box::new(Node {
            val: 1,
            children: vec![
                Box::new(Node { val: 2, children: vec![Box::new(Node { val: 3, children: vec![] })] }),
                Box::new(Node { val: 4, children: vec![] }),
            ],
        }));
        assert_eq!(diameter(root), 3);
    }
}
