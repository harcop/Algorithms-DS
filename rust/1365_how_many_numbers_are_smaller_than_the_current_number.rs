/// LeetCode #1365 - How Many Numbers Are Smaller Than The Current Number

fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut sorted = nums.clone();
    sorted.sort_unstable();
    nums.iter()
        .map(|&x| sorted.partition_point(|&v| v < x) as i32)
        .collect()
}

fn main() {
    println!("{:?}", smaller_numbers_than_current(vec![8, 1, 2, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::smaller_numbers_than_current;

    #[test]
    fn example_one() {
        assert_eq!(smaller_numbers_than_current(vec![8, 1, 2, 2, 3]), vec![4, 0, 1, 1, 3]);
    }

    #[test]
    fn example_two() {
        assert_eq!(smaller_numbers_than_current(vec![6, 5, 4, 8]), vec![2, 1, 0, 3]);
    }
}
