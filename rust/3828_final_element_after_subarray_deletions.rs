/// LeetCode #3828 - Final Element After Subarray Deletions
fn final_element(nums: Vec<i32>) -> i32 {
    nums[0].max(*nums.last().unwrap())
}

fn main() {
    println!("{}", final_element(vec![1, 5, 2]));
}

#[cfg(test)]
mod tests {
    use super::final_element;

    #[test]
    fn example1() {
        assert_eq!(final_element(vec![1, 5, 2]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(final_element(vec![3, 7]), 7);
    }
}
