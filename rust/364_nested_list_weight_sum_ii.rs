/// LeetCode #364 - Nested List Weight Sum II (deepest level weight 1)
#[derive(Debug, Clone)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

fn depth_sum_inverse(nested_list: Vec<NestedInteger>) -> i32 {
    let mut ints: Vec<(i32, i32)> = Vec::new();
    fn walk(items: &[NestedInteger], depth: i32, out: &mut Vec<(i32, i32)>) {
        for it in items {
            match it {
                NestedInteger::Int(v) => out.push((*v, depth)),
                NestedInteger::List(v) => walk(v, depth + 1, out),
            }
        }
    }
    walk(&nested_list, 1, &mut ints);
    let mx = ints.iter().map(|&(_, d)| d).max().unwrap_or(1);
    ints.into_iter().map(|(v, d)| v * (mx - d + 1)).sum()
}

fn main() {
    println!(
        "{}",
        depth_sum_inverse(vec![
            NestedInteger::Int(1),
            NestedInteger::List(vec![
                NestedInteger::Int(4),
                NestedInteger::List(vec![NestedInteger::Int(6)]),
            ]),
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lc_example() {
        let v = vec![
            NestedInteger::Int(1),
            NestedInteger::List(vec![
                NestedInteger::Int(4),
                NestedInteger::List(vec![NestedInteger::Int(6)]),
            ]),
        ];
        assert_eq!(depth_sum_inverse(v), 17);
    }
}
