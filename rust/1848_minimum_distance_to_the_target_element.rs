/// LeetCode #1848 - Minimum Distance to the Target Element
fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
    nums.iter()
        .enumerate()
        .filter(|(_, &x)| x == target)
        .map(|(i, _)| ((i as i32) - start).abs())
        .min()
        .unwrap()
}

fn main() {
    println!("{}", get_min_distance(vec![1, 2, 3, 4, 5], 5, 3));
}

#[cfg(test)]
mod tests {
    use super::get_min_distance;

    #[test]
    fn example_one() {
        assert_eq!(get_min_distance(vec![1, 2, 3, 4, 5], 5, 3), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(get_min_distance(vec![1], 1, 0), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(get_min_distance(vec![1; 10], 1, 0), 0);
    }
}
