/// LeetCode #2270 - Number of Ways to Split Array
fn ways_to_split_array(nums: Vec<i32>) -> i32 {
    let mut prefix: i64 = 0;
    let mut suffix: i64 = nums.iter().map(|&x| x as i64).sum();
    let mut ans = 0;

    for i in 0..nums.len() - 1 {
        prefix += nums[i] as i64;
        suffix -= nums[i] as i64;
        if prefix >= suffix {
            ans += 1;
        }
    }

    ans
}

fn main() {
    println!("{}", ways_to_split_array(vec![10, 4, -8, 7]));
}

#[cfg(test)]
mod tests {
    use super::ways_to_split_array;

    #[test]
    fn example_one() {
        assert_eq!(ways_to_split_array(vec![10, 4, -8, 7]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(ways_to_split_array(vec![2, 3, 1, 0]), 2);
    }
}
