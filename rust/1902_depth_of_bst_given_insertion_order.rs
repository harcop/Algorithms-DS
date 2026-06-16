/// LeetCode #1902 - Depth of BST Given Insertion Order
use std::collections::BTreeMap;

fn max_depth_bst(order: Vec<i32>) -> i32 {
    let mut sd: BTreeMap<i32, i32> = BTreeMap::new();
    sd.insert(0, 0);
    sd.insert(i32::MAX, 0);
    sd.insert(order[0], 1);
    let mut ans = 1;
    for &v in order.iter().skip(1) {
        let lower = *sd.range(..v).next_back().unwrap().1;
        let higher = *sd.range(v..).next().unwrap().1;
        let depth = 1 + lower.max(higher);
        ans = ans.max(depth);
        sd.insert(v, depth);
    }
    ans
}

fn main() {
    println!("{}", max_depth_bst(vec![2, 1, 4, 3]));
}

#[cfg(test)]
mod tests {
    use super::max_depth_bst;

    #[test]
    fn example_one() {
        assert_eq!(max_depth_bst(vec![2, 1, 4, 3]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_depth_bst(vec![2, 1]), 2);
    }
}
