/// LeetCode #1650 - Lowest Common Ancestor Of A Binary Tree Iii
use std::collections::HashMap;

pub struct Node {
    pub val: i32,
    pub parent: Option<i32>,
}

fn lowest_common_ancestor(p: i32, q: i32, parent: &HashMap<i32, i32>) -> i32 {
    let mut seen = std::collections::HashSet::new();
    let mut cur = p;
    loop {
        seen.insert(cur);
        cur = match parent.get(&cur) {
            Some(&par) => par,
            None => break,
        };
    }
    let mut cur = q;
    loop {
        if seen.contains(&cur) { return cur; }
        cur = match parent.get(&cur) {
            Some(&par) => par,
            None => break,
        };
    }
    -1
}
fn main() {
    let mut parent = HashMap::new();
    parent.insert(5, 3); parent.insert(3, 1); parent.insert(1, 0);
    println!("{}", lowest_common_ancestor(5, 1, &parent));
}
#[cfg(test)]
mod tests {
    use super::lowest_common_ancestor;
    use std::collections::HashMap;
    #[test]
    fn example_one() {
        let mut parent = HashMap::new();
        parent.insert(5, 3); parent.insert(3, 1); parent.insert(1, 0);
        assert_eq!(lowest_common_ancestor(5, 1, &parent), 1);
    }
}