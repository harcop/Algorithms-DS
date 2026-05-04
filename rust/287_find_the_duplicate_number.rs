/// LeetCode #287 - Find the Duplicate Number (Floyd cycle)
fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut slow = nums[0] as usize;
    let mut fast = nums[0] as usize;
    loop {
        slow = nums[slow] as usize;
        fast = nums[nums[fast] as usize] as usize;
        if slow == fast {
            break;
        }
    }
    // Phase 2: one pointer from start, one from meeting point (see LC proof).
    fast = nums[0] as usize;
    while slow != fast {
        slow = nums[slow] as usize;
        fast = nums[fast] as usize;
    }
    slow as i32
}

fn main() {
    println!("{}", find_duplicate(vec![1, 3, 4, 2, 2]));
}

#[cfg(test)]
mod tests {
    use super::find_duplicate;

    #[test]
    fn example_one() {
        assert_eq!(find_duplicate(vec![1, 3, 4, 2, 2]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_duplicate(vec![3, 1, 3, 4, 2]), 3);
    }
}
