/// LeetCode #3668 - Restore Finishing Order
use std::collections::HashSet;

fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {
    let set: HashSet<i32> = friends.into_iter().collect();
    order.into_iter().filter(|x| set.contains(x)).collect()
}

fn main() {
    println!("{:?}", recover_order(vec![3, 1, 2, 5, 4], vec![1, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::recover_order;

    #[test]
    fn example1() {
        assert_eq!(recover_order(vec![3, 1, 2, 5, 4], vec![1, 3, 4]), vec![3, 1, 4]);
    }

    #[test]
    fn example2() {
        assert_eq!(recover_order(vec![1, 4, 5, 3, 2], vec![2, 5]), vec![5, 2]);
    }
}
