/// LeetCode #2576 - Find the Maximum Number of Marked Indices
fn max_num_of_marked_indices(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    let mut i = 0usize;
    for j in (n + 1) / 2..n {
        if nums[i] as i64 * 2 <= nums[j] as i64 {
            i += 1;
        }
    }
    (i * 2) as i32
}

fn main() {
    println!("{}", max_num_of_marked_indices(vec![3, 5, 2, 4]));
}

#[cfg(test)]
mod tests {
    use super::max_num_of_marked_indices;

    #[test]
    fn example_one() {
        assert_eq!(max_num_of_marked_indices(vec![3, 5, 2, 4]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_num_of_marked_indices(vec![9, 2, 5, 4]), 4);
    }

    #[test]
    fn example_three() {
        assert_eq!(max_num_of_marked_indices(vec![7, 6, 8]), 0);
    }
}
