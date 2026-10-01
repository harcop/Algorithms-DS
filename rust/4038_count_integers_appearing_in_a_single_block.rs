/// LeetCode #4038 - Count Integers Appearing in a Single Block
fn count_special_integers(nums: Vec<i32>) -> i32 {
    let mut runs = std::collections::HashMap::new();
    for i in 0..nums.len() {
        if i == 0 || nums[i] != nums[i - 1] {
            *runs.entry(nums[i]).or_insert(0) += 1;
        }
    }
    runs.values().filter(|&&c| c == 1).count() as i32
}

fn main() {
    println!("{}", count_special_integers(vec![1, 2, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::count_special_integers;

    #[test]
    fn example1() {
        assert_eq!(count_special_integers(vec![1, 2, 2, 1]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(count_special_integers(vec![3, 3, 1, 2, 2, 1]), 2);
    }
}
