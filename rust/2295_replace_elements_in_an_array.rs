/// LeetCode #2295 - Replace Elements in an Array
use std::collections::HashMap;

fn array_change(mut nums: Vec<i32>, operations: Vec<Vec<i32>>) -> Vec<i32> {
    let mut pos: HashMap<i32, usize> = HashMap::new();
    for (i, &x) in nums.iter().enumerate() {
        pos.insert(x, i);
    }
    for op in operations {
        let (old, new) = (op[0], op[1]);
        let i = pos.remove(&old).unwrap();
        nums[i] = new;
        pos.insert(new, i);
    }
    nums
}

fn main() {
    println!("{:?}", array_change(vec![1, 2, 4, 6], vec![vec![1, 3], vec![4, 7], vec![6, 1]]));
}

#[cfg(test)]
mod tests {
    use super::array_change;

    #[test]
    fn example_one() {
        assert_eq!(
            array_change(vec![1, 2, 4, 6], vec![vec![1, 3], vec![4, 7], vec![6, 1]]),
            vec![3, 2, 7, 1]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            array_change(vec![1, 2], vec![vec![1, 3], vec![2, 1], vec![3, 2]]),
            vec![2, 1]
        );
    }
}
