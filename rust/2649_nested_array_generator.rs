/// LeetCode #2649 - Nested Array Generator (JS problem; Rust nested enum analogue)
#[derive(Clone, Debug)]
enum Nested {
    Int(i32),
    Arr(Vec<Nested>),
}

fn inorder_traversal(arr: &[Nested]) -> Vec<i32> {
    let mut out = Vec::new();
    fn walk(items: &[Nested], out: &mut Vec<i32>) {
        for e in items {
            match e {
                Nested::Int(x) => out.push(*x),
                Nested::Arr(a) => walk(a, out),
            }
        }
    }
    walk(arr, &mut out);
    out
}

fn main() {
    let arr = vec![
        Nested::Arr(vec![Nested::Arr(vec![Nested::Int(6)])]),
        Nested::Arr(vec![Nested::Int(1), Nested::Int(3)]),
        Nested::Arr(vec![]),
    ];
    println!("{:?}", inorder_traversal(&arr));
}

#[cfg(test)]
mod tests {
    use super::{inorder_traversal, Nested};

    #[test]
    fn example_one() {
        let arr = vec![
            Nested::Arr(vec![Nested::Arr(vec![Nested::Int(6)])]),
            Nested::Arr(vec![Nested::Int(1), Nested::Int(3)]),
            Nested::Arr(vec![]),
        ];
        assert_eq!(inorder_traversal(&arr), vec![6, 1, 3]);
    }

    #[test]
    fn example_two() {
        assert!(inorder_traversal(&[]).is_empty());
    }

    #[test]
    fn flat_then_nested() {
        let arr = vec![
            Nested::Int(1),
            Nested::Arr(vec![Nested::Int(2), Nested::Int(3)]),
        ];
        assert_eq!(inorder_traversal(&arr), vec![1, 2, 3]);
    }
}
