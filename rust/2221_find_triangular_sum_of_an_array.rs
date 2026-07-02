/// LeetCode #2221 - Find Triangular Sum of an Array
fn triangular_sum(mut nums: Vec<i32>) -> i32 {
    let mut sz = nums.len();
    while sz > 0 {
        for i in 0..sz - 1 {
            nums[i] = (nums[i] + nums[i + 1]) % 10;
        }
        sz -= 1;
    }
    nums[0]
}

fn main() {
    println!("{}", triangular_sum(vec![1, 2, 3, 4, 5]));
}

#[cfg(test)]
mod tests {
    use super::triangular_sum;

    #[test]
    fn example_one() {
        assert_eq!(triangular_sum(vec![1, 2, 3, 4, 5]), 8);
    }

    #[test]
    fn example_two() {
        assert_eq!(triangular_sum(vec![3]), 3);
    }
}
