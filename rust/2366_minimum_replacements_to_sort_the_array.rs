/// LeetCode #2366 - Minimum Replacements to Sort the Array
fn minimum_replacement(nums: Vec<i32>) -> i64 {
    let mut ans = 0i64;
    let mut mx = nums[nums.len() - 1];

    for i in (0..nums.len() - 1).rev() {
        let ops = (nums[i] - 1) / mx;
        ans += ops as i64;
        mx = nums[i] / (ops + 1);
    }

    ans
}

fn main() {
    println!("{}", minimum_replacement(vec![3, 9, 3]));
}

#[cfg(test)]
mod tests {
    use super::minimum_replacement;

    #[test]
    fn example_one() {
        assert_eq!(minimum_replacement(vec![3, 9, 3]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_replacement(vec![1, 2, 3, 4, 5]), 0);
    }
}
