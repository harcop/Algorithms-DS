/// LeetCode #11 - Container With Most Water
///
/// Two-pointer sweep from both ends.
fn max_area(height: Vec<i32>) -> i32 {
    if height.len() < 2 {
        return 0;
    }

    let mut left = 0usize;
    let mut right = height.len() - 1;
    let mut best = 0i32;

    while left < right {
        let h = height[left].min(height[right]);
        let w = (right - left) as i32;
        best = best.max(h * w);

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    best
}

fn main() {
    println!("{}", max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
}

#[cfg(test)]
mod tests {
    use super::max_area;

    #[test]
    fn example_one() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_area(vec![1, 1]), 1);
    }
}
