/// LeetCode #2219 - Maximum Sum Score of Array
fn maximum_sum_score(nums: Vec<i32>) -> i64 {
    let sum: i64 = nums.iter().map(|&x| x as i64).sum();
    let mut prefix = 0i64;
    let mut ans = i64::MIN;

    for &num in &nums {
        prefix += num as i64;
        ans = ans.max(prefix).max(sum - prefix + num as i64);
    }

    ans
}

fn main() {
    println!("{}", maximum_sum_score(vec![-5, 1, 5, -5]));
}

#[cfg(test)]
mod tests {
    use super::maximum_sum_score;

    #[test]
    fn example_one() {
        assert_eq!(maximum_sum_score(vec![-5, 1, 5, -5]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_sum_score(vec![-3, -5]), -3);
    }
}
