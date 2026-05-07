/// LeetCode #339 - Nested List Weight Sum
#[derive(Debug, Clone)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

fn depth_sum(nested_list: Vec<NestedInteger>) -> i32 {
    fn walk(items: &[NestedInteger], depth: i32) -> i32 {
        let mut acc = 0;
        for it in items {
            match it {
                NestedInteger::Int(x) => acc += x * depth,
                NestedInteger::List(v) => acc += walk(v, depth + 1),
            }
        }
        acc
    }
    walk(&nested_list, 1)
}

fn main() {
    let lst = vec![
        NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
        NestedInteger::Int(2),
        NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
    ];
    println!("{}", depth_sum(lst));
}

#[cfg(test)]
mod tests {
    use super::{depth_sum, NestedInteger};

    #[test]
    fn examples() {
        let nested = vec![
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
            NestedInteger::Int(2),
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
        ];
        assert_eq!(depth_sum(nested), 10);
    }
}
