/// LeetCode #915 - Partition Array into Disjoint Intervals
fn partition_disjoint(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut right_min = vec![0i32; n];
    right_min[n - 1] = nums[n - 1];
    for i in (0..n - 1).rev() {
        right_min[i] = nums[i].min(right_min[i + 1]);
    }
    let mut left_max = nums[0];
    for i in 0..n - 1 {
        if left_max <= right_min[i + 1] {
            return (i + 1) as i32;
        }
        left_max = left_max.max(nums[i + 1]);
    }
    (n - 1) as i32
}

fn main() {
    println!("{}", partition_disjoint(vec![5, 0, 3, 8, 6]));
}

#[cfg(test)]
mod tests {
    use super::partition_disjoint;

    #[test]
    fn example_one() {
        assert_eq!(partition_disjoint(vec![5, 0, 3, 8, 6]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(partition_disjoint(vec![1, 1, 1, 0, 6, 12]), 4);
    }
}
