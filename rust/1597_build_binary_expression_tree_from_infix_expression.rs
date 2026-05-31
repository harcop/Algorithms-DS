/// LeetCode #1597 - Build Binary Expression Tree From Infix Expression
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub val: String,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

fn prec(c: u8) -> i32 {
    match c {
        b'+' | b'-' => 1,
        b'*' | b'/' => 2,
        _ => 0,
    }
}

fn apply(op: u8, a: Box<Node>, b: Box<Node>) -> Box<Node> {
    Box::new(Node {
        val: String::from(op as char),
        left: Some(a),
        right: Some(b),
    })
}

fn exp_tree(s: String) -> Option<Box<Node>> {
    let b = s.as_bytes();
    let mut ops: Vec<u8> = vec![];
    let mut st: Vec<Box<Node>> = vec![];
    let mut i = 0usize;
    while i < b.len() {
        let c = b[i];
        if c.is_ascii_digit() {
            let mut j = i;
            while j < b.len() && b[j].is_ascii_digit() {
                j += 1;
            }
            st.push(Box::new(Node {
                val: String::from_utf8(b[i..j].to_vec()).unwrap(),
                left: None,
                right: None,
            }));
            i = j;
        } else if c == b'(' {
            ops.push(c);
            i += 1;
        } else if c == b')' {
            while ops.last().copied() != Some(b'(') {
                let op = ops.pop().unwrap();
                let r = st.pop().unwrap();
                let l = st.pop().unwrap();
                st.push(apply(op, l, r));
            }
            ops.pop();
            i += 1;
        } else {
            while ops.last().map(|&o| prec(o) >= prec(c)).unwrap_or(false) {
                let op = ops.pop().unwrap();
                let r = st.pop().unwrap();
                let l = st.pop().unwrap();
                st.push(apply(op, l, r));
            }
            ops.push(c);
            i += 1;
        }
    }
    while let Some(op) = ops.pop() {
        let r = st.pop().unwrap();
        let l = st.pop().unwrap();
        st.push(apply(op, l, r));
    }
    st.pop()
}

fn main() {
    println!("{:?}", exp_tree("3*4-2+5".into()).unwrap().val);
}

#[cfg(test)]
mod tests {
    use super::exp_tree;

    #[test]
    fn example_one() {
        assert!(exp_tree("3*4-2+5".into()).is_some());
    }
}
