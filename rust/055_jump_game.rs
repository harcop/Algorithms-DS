/// LeetCode #55 - Jump Game
fn can_jump(nums: Vec<i32>) -> bool {
    let mut farthest = 0usize;

    for (i, &step) in nums.iter().enumerate() {
        if i > farthest {
            return false;
        }
        farthest = farthest.max(i + step as usize);
        if farthest >= nums.len().saturating_sub(1) {
            return true;
        }
    }

    true
}

fn main() {
    println!("{}", can_jump(vec![2, 3, 1, 1, 4]));
}

#[cfg(test)]
mod tests {
    use super::can_jump;

    #[test]
    fn example_one() {
        assert!(can_jump(vec![2, 3, 1, 1, 4]));
    }

    #[test]
    fn example_two() {
        assert!(!can_jump(vec![3, 2, 1, 0, 4]));
    }
}
