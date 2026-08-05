/// LeetCode #3012 - Minimize Length of Array Using Operations
fn minimum_array_length(nums: Vec<i32>) -> i32 {
    let mi = *nums.iter().min().unwrap();
    if nums.iter().any(|&x| x % mi != 0) {
        return 1;
    }
    let cnt = nums.iter().filter(|&&x| x == mi).count();
    ((cnt + 1) / 2) as i32
}

fn main() {
    println!("{}", minimum_array_length(vec![1, 4, 3, 1]));
    println!("{}", minimum_array_length(vec![5, 5, 5, 10, 5]));
    println!("{}", minimum_array_length(vec![2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::minimum_array_length;

    #[test]
    fn example_one() {
        assert_eq!(minimum_array_length(vec![1, 4, 3, 1]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_array_length(vec![5, 5, 5, 10, 5]), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_array_length(vec![2, 3, 4]), 1);
    }
}
