/// LeetCode #413 - Arithmetic Slices
fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    if nums.len() < 3 {
        return 0;
    }
    let mut cur = 0i32;
    let mut ans = 0i32;
    for i in 2..nums.len() {
        if nums[i] - nums[i - 1] == nums[i - 1] - nums[i - 2] {
            cur += 1;
            ans += cur;
        } else {
            cur = 0;
        }
    }
    ans
}

fn main() {
    println!("{}", number_of_arithmetic_slices(vec![1, 2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::number_of_arithmetic_slices;

    #[test]
    fn example_one() {
        assert_eq!(number_of_arithmetic_slices(vec![1, 2, 3, 4]), 3);
    }
}
