/// LeetCode #215 - Kth Largest Element in an Array
fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    nums.sort_unstable();
    nums[nums.len() - k]
}

fn main() {
    println!("{}", find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2));
}

#[cfg(test)]
mod tests {
    use super::find_kth_largest;

    #[test]
    fn example_one() {
        assert_eq!(find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
    }
}
