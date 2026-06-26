/// LeetCode #2091 - Removing Minimum and Maximum From Array
fn minimum_deletions(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut mn = 0usize;
    let mut mx = 0usize;
    for i in 1..n {
        if nums[i] < nums[mn] {
            mn = i;
        }
        if nums[i] > nums[mx] {
            mx = i;
        }
    }

    let left = mn.min(mx);
    let right = mn.max(mx);
    let from_front = right + 1;
    let from_back = n - left;
    let both_sides = left + 1 + n - right;
    from_front.min(from_back).min(both_sides) as i32
}

fn main() {
    println!("{}", minimum_deletions(vec![2, 10, 7, 5, 4, 1, 8, 6]));
}

#[cfg(test)]
mod tests {
    use super::minimum_deletions;

    #[test]
    fn example_one() {
        assert_eq!(minimum_deletions(vec![2, 10, 7, 5, 4, 1, 8, 6]), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_deletions(vec![0, -4, 19, 1, 8, -2, -3, 5]), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_deletions(vec![101]), 1);
    }
}
