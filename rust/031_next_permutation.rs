/// LeetCode #31 - Next Permutation
fn next_permutation(nums: &mut [i32]) {
    let n = nums.len();
    if n < 2 {
        return;
    }

    let mut i = n - 1;
    while i > 0 && nums[i - 1] >= nums[i] {
        i -= 1;
    }

    if i > 0 {
        let pivot = i - 1;
        let mut j = n - 1;
        while nums[j] <= nums[pivot] {
            j -= 1;
        }
        nums.swap(pivot, j);
    }

    nums[i..].reverse();
}

fn main() {
    let mut nums = vec![1, 2, 3];
    next_permutation(&mut nums);
    println!("{nums:?}");
}

#[cfg(test)]
mod tests {
    use super::next_permutation;

    #[test]
    fn example_one() {
        let mut nums = vec![1, 2, 3];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);
    }

    #[test]
    fn example_two() {
        let mut nums = vec![3, 2, 1];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn example_three() {
        let mut nums = vec![1, 1, 5];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 5, 1]);
    }
}
