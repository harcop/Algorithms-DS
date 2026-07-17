/// LeetCode #2460 - Apply Operations to an Array
fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
    for i in 0..nums.len() - 1 {
        if nums[i] == nums[i + 1] {
            nums[i] *= 2;
            nums[i + 1] = 0;
        }
    }

    let mut answer: Vec<i32> = nums.iter().copied().filter(|&num| num != 0).collect();
    answer.resize(nums.len(), 0);
    answer
}

fn main() {
    println!("{:?}", apply_operations(vec![1, 2, 2, 1, 1, 0]));
}

#[cfg(test)]
mod tests {
    use super::apply_operations;

    #[test]
    fn example_one() {
        assert_eq!(
            apply_operations(vec![1, 2, 2, 1, 1, 0]),
            vec![1, 4, 2, 0, 0, 0]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(apply_operations(vec![0, 1]), vec![1, 0]);
    }
}
