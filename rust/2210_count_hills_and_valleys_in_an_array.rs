/// LeetCode #2210 - Count Hills and Valleys in an Array
fn count_hill_valley(nums: Vec<i32>) -> i32 {
    let mut ans = 0i32;
    let mut left = nums[0];

    for i in 1..nums.len() - 1 {
        if (left < nums[i] && nums[i] > nums[i + 1]) || (left > nums[i] && nums[i] < nums[i + 1]) {
            ans += 1;
            left = nums[i];
        }
    }

    ans
}

fn main() {
    println!("{}", count_hill_valley(vec![2, 4, 1, 1, 6, 5]));
}

#[cfg(test)]
mod tests {
    use super::count_hill_valley;

    #[test]
    fn example_one() {
        assert_eq!(count_hill_valley(vec![2, 4, 1, 1, 6, 5]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_hill_valley(vec![6, 6, 5, 5, 4, 1]), 0);
    }
}
