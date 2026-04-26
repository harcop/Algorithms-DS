/// LeetCode #41 - First Missing Positive
fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();

    for i in 0..n {
        while nums[i] > 0
            && (nums[i] as usize) <= n
            && nums[i] != nums[(nums[i] - 1) as usize]
        {
            let idx = (nums[i] - 1) as usize;
            nums.swap(i, idx);
        }
    }

    for (i, &v) in nums.iter().enumerate() {
        if v != (i + 1) as i32 {
            return (i + 1) as i32;
        }
    }

    (n + 1) as i32
}

fn main() {
    println!("{}", first_missing_positive(vec![1, 2, 0]));
}

#[cfg(test)]
mod tests {
    use super::first_missing_positive;

    #[test]
    fn example_one() {
        assert_eq!(first_missing_positive(vec![1, 2, 0]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(first_missing_positive(vec![3, 4, -1, 1]), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    }
}
