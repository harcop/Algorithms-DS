/// LeetCode #45 - Jump Game II
fn jump(nums: Vec<i32>) -> i32 {
    if nums.len() <= 1 {
        return 0;
    }

    let mut jumps = 0;
    let mut current_end = 0usize;
    let mut farthest = 0usize;

    for (i, &step) in nums.iter().enumerate().take(nums.len() - 1) {
        farthest = farthest.max(i + step as usize);
        if i == current_end {
            jumps += 1;
            current_end = farthest;
        }
    }

    jumps
}

fn main() {
    println!("{}", jump(vec![2, 3, 1, 1, 4]));
}

#[cfg(test)]
mod tests {
    use super::jump;

    #[test]
    fn example_one() {
        assert_eq!(jump(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(jump(vec![2, 3, 0, 1, 4]), 2);
    }
}
