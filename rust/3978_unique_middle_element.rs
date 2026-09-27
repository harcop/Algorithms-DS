/// LeetCode #3978 - Unique Middle Element
fn is_middle_element_unique(nums: Vec<i32>) -> bool {
    let mid = nums[nums.len() / 2];
    nums.iter().filter(|&&x| x == mid).count() == 1
}

fn main() {
    println!("{}", is_middle_element_unique(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::is_middle_element_unique;

    #[test]
    fn example1() {
        assert!(is_middle_element_unique(vec![1, 2, 3]));
    }

    #[test]
    fn example2() {
        assert!(!is_middle_element_unique(vec![1, 2, 2]));
    }
}
