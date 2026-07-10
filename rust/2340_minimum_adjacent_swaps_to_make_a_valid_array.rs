/// LeetCode #2340 - Minimum Adjacent Swaps to Make a Valid Array
fn minimum_swaps(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut i = 0usize;
    let mut j = 0usize;

    for k in 0..n {
        if nums[k] < nums[i] || (nums[k] == nums[i] && k < i) {
            i = k;
        }
        if nums[k] > nums[j] || (nums[k] == nums[j] && k > j) {
            j = k;
        }
    }

    if i == j {
        return 0;
    }
    (i + n - 1 - j - if i > j { 1 } else { 0 }) as i32
}

fn main() {
    println!("{}", minimum_swaps(vec![3, 4, 5, 5, 3, 1]));
}

#[cfg(test)]
mod tests {
    use super::minimum_swaps;

    #[test]
    fn example_one() {
        assert_eq!(minimum_swaps(vec![3, 4, 5, 5, 3, 1]), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_swaps(vec![9]), 0);
    }
}
