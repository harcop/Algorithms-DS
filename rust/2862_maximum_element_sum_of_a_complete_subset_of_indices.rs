/// LeetCode #2862 - Maximum Element-Sum of a Complete Subset of Indices
fn maximum_sum(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut answer = 0i64;

    for base in 1..=n {
        let mut sum = 0i64;
        let mut multiplier = 1usize;
        while base * multiplier * multiplier <= n {
            sum += nums[base * multiplier * multiplier - 1] as i64;
            multiplier += 1;
        }
        answer = answer.max(sum);
    }
    answer
}

fn main() {
    println!("{}", maximum_sum(vec![8, 7, 3, 5, 7, 2, 4, 9]));
}

#[cfg(test)]
mod tests {
    use super::maximum_sum;

    #[test]
    fn example_one() {
        assert_eq!(maximum_sum(vec![8, 7, 3, 5, 7, 2, 4, 9]), 16);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_sum(vec![8, 10, 3, 8, 1, 13, 7, 9, 4]), 20);
    }
}
