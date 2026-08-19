/// LeetCode #3285 - Find Indices of Stable Mountains
fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
    (1..height.len())
        .filter(|&i| height[i - 1] > threshold)
        .map(|i| i as i32)
        .collect()
}

fn main() {
    println!("{:?}", stable_mountains(vec![1, 2, 3, 4, 5], 2));
}

#[cfg(test)]
mod tests {
    use super::stable_mountains;

    #[test]
    fn example1() {
        assert_eq!(stable_mountains(vec![1, 2, 3, 4, 5], 2), vec![3, 4]);
    }

    #[test]
    fn example2() {
        assert_eq!(stable_mountains(vec![10, 1, 10, 1, 10], 3), vec![1, 3]);
    }

    #[test]
    fn example3() {
        assert_eq!(stable_mountains(vec![10, 1, 10, 1, 10], 10), vec![] as Vec<i32>);
    }
}
