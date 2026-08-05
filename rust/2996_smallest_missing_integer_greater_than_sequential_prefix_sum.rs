/// LeetCode #2996 - Smallest Missing Integer Greater Than Sequential Prefix Sum
use std::collections::HashSet;

fn find_smallest_integer(nums: Vec<i32>) -> i32 {
    let set: HashSet<i32> = nums.iter().copied().collect();
    let mut prefix_sum = nums[0];
    for i in 1..nums.len() {
        if nums[i] != nums[i - 1] + 1 {
            break;
        }
        prefix_sum += nums[i];
    }
    let mut x = prefix_sum;
    while set.contains(&x) {
        x += 1;
    }
    x
}

fn main() {
    println!("{}", find_smallest_integer(vec![1, 2, 3, 2, 5]));
}

#[cfg(test)]
mod tests {
    use super::find_smallest_integer;

    #[test]
    fn example_one() {
        assert_eq!(find_smallest_integer(vec![1, 2, 3, 2, 5]), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_smallest_integer(vec![3, 4, 5, 1, 12, 14, 13]), 15);
    }
}
