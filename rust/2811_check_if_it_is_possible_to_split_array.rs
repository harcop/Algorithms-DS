/// LeetCode #2811 - Check if it is Possible to Split Array
fn can_split_array(nums: Vec<i32>, m: i32) -> bool {
    let n = nums.len();
    if n <= 2 {
        return true;
    }
    for i in 1..n {
        if nums[i - 1] + nums[i] >= m {
            return true;
        }
    }
    false
}

fn main() {
    println!("{}", can_split_array(vec![2, 2, 1], 4));
}

#[cfg(test)]
mod tests {
    use super::can_split_array;

    #[test]
    fn example_one() {
        assert!(can_split_array(vec![2, 2, 1], 4));
    }

    #[test]
    fn example_two() {
        assert!(!can_split_array(vec![2, 1, 3], 5));
    }

    #[test]
    fn example_three() {
        assert!(can_split_array(vec![2, 3, 3, 2, 3], 6));
    }
}
