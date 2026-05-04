/// LeetCode #280 - Wiggle Sort
fn wiggle_sort(nums: &mut Vec<i32>) {
    nums.sort_unstable();
    for i in (1..nums.len().saturating_sub(1)).step_by(2) {
        nums.swap(i, i + 1);
    }
}

fn main() {
    let mut v = vec![3, 5, 2, 1, 6, 4];
    wiggle_sort(&mut v);
    println!("{:?}", v);
}

#[cfg(test)]
mod tests {
    use super::wiggle_sort;

    fn ok(nums: &[i32]) -> bool {
        for i in 0..nums.len() - 1 {
            if i % 2 == 0 {
                if nums[i] >= nums[i + 1] {
                    return false;
                }
            } else if nums[i] <= nums[i + 1] {
                return false;
            }
        }
        true
    }

    #[test]
    fn example_one() {
        let mut v = vec![3, 5, 2, 1, 6, 4];
        wiggle_sort(&mut v);
        assert!(ok(&v));
    }
}
