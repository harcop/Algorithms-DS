/// LeetCode #341 - Flatten Nested List Iterator
#[derive(Debug, Clone)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

pub struct NestedIterator {
    stack: Vec<std::vec::IntoIter<NestedInteger>>,
    next_int: Option<i32>,
}

impl NestedIterator {
    pub fn new(mut nested_list: Vec<NestedInteger>) -> Self {
        let mut s = NestedIterator {
            stack: vec![],
            next_int: None,
        };
        nested_list.reverse();
        s.stack.push(nested_list.into_iter());
        s.advance();
        s
    }

    fn advance(&mut self) {
        self.next_int = None;
        while let Some(mut it) = self.stack.pop() {
            match it.next() {
                Some(NestedInteger::Int(v)) => {
                    self.stack.push(it);
                    self.next_int = Some(v);
                    break;
                }
                Some(NestedInteger::List(mut inner)) => {
                    self.stack.push(it);
                    inner.reverse();
                    self.stack.push(inner.into_iter());
                }
                None => {}
            }
        }
    }

    pub fn next(&mut self) -> i32 {
        let v = self.next_int.expect("empty");
        self.advance();
        v
    }

    pub fn has_next(&self) -> bool {
        self.next_int.is_some()
    }
}

fn main() {
    let lst = vec![
        NestedInteger::Int(1),
        NestedInteger::List(vec![NestedInteger::Int(2), NestedInteger::List(vec![NestedInteger::Int(3)])]),
    ];
    let mut it = NestedIterator::new(lst);
    while it.has_next() {
        print!("{} ", it.next());
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::{NestedInteger, NestedIterator};

    #[test]
    fn example() {
        let lst = vec![
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
            NestedInteger::Int(2),
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
        ];
        let mut it = NestedIterator::new(lst);
        let mut got = vec![];
        while it.has_next() {
            got.push(it.next());
        }
        assert_eq!(got, vec![1, 1, 2, 1, 1]);
    }
}
