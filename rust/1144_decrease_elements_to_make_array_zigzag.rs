/// LeetCode #1144 - Decrease Elements To Make Array Zigzag
fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
    let mut cost0 = 0i32;
    let mut cost1 = 0i32;
    for i in 0..nums.len() {
        let left = if i > 0 { nums[i - 1] } else { i32::MAX };
        let right = if i + 1 < nums.len() { nums[i + 1] } else { i32::MAX };
        if i % 2 == 0 {
            cost0 += (nums[i] - (left.min(right) - 1)).max(0);
        } else {
            cost1 += (nums[i] - (left.min(right) - 1)).max(0);
        }
    }
    cost0.min(cost1)
}

fn main() {
    println!("{}", moves_to_make_zigzag(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::moves_to_make_zigzag;

    #[test]
    fn example_one() {
        assert_eq!(moves_to_make_zigzag(vec![1, 2, 3]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(moves_to_make_zigzag(vec![9, 6, 1, 6, 2]), 4);
    }
}
