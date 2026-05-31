/// LeetCode #1628 - Design An Expression Tree With Evaluate Function
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub val: String,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

pub struct ExpTree {
    root: Option<Box<Node>>,
}

impl ExpTree {
    fn new(s: String) -> Self {
        let mut st: Vec<Box<Node>> = vec![];
        for tok in s.split_whitespace() {
            if tok.chars().all(|c| c.is_ascii_digit()) {
                st.push(Box::new(Node {
                    val: tok.to_string(),
                    left: None,
                    right: None,
                }));
            } else {
                let right = st.pop().unwrap();
                let left = st.pop().unwrap();
                st.push(Box::new(Node {
                    val: tok.to_string(),
                    left: Some(left),
                    right: Some(right),
                }));
            }
        }
        ExpTree { root: st.pop() }
    }

    fn evaluate(&self) -> i32 {
        fn eval(n: &Node) -> i64 {
            if n.left.is_none() {
                return n.val.parse().unwrap();
            }
            let l = eval(n.left.as_ref().unwrap());
            let r = eval(n.right.as_ref().unwrap());
            match n.val.as_str() {
                "+" => l + r,
                "-" => l - r,
                "*" => l * r,
                "/" => l / r,
                _ => 0,
            }
        }
        eval(self.root.as_ref().unwrap()) as i32
    }
}

fn main() {
    let t = ExpTree::new("3 4 + 2 *".into());
    println!("{}", t.evaluate());
}

#[cfg(test)]
mod tests {
    use super::ExpTree;

    #[test]
    fn example_one() {
        let t = ExpTree::new("3 4 + 2 *".into());
        assert_eq!(t.evaluate(), 14);
    }
}
