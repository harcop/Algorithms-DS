/// LeetCode #1920 - Build Array from Permutation
fn build_array(nums: Vec<i32>) -> Vec<i32> {
    nums.iter().map(|&i| nums[i as usize]).collect()
}

fn main() {
    println!("{:?}", build_array(vec![0, 2, 1, 5, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::build_array;

    #[test]
    fn example_one() {
        assert_eq!(build_array(vec![0, 2, 1, 5, 3, 4]), vec![0, 1, 2, 4, 5, 3]);
    }
}
