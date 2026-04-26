/// LeetCode #42 - Trapping Rain Water
fn trap(height: Vec<i32>) -> i32 {
    if height.len() < 3 {
        return 0;
    }

    let mut left = 0usize;
    let mut right = height.len() - 1;
    let mut left_max = 0;
    let mut right_max = 0;
    let mut water = 0;

    while left < right {
        if height[left] < height[right] {
            if height[left] >= left_max {
                left_max = height[left];
            } else {
                water += left_max - height[left];
            }
            left += 1;
        } else {
            if height[right] >= right_max {
                right_max = height[right];
            } else {
                water += right_max - height[right];
            }
            right -= 1;
        }
    }

    water
}

fn main() {
    println!("{}", trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::trap;

    #[test]
    fn example_one() {
        assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
