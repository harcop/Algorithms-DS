/// LeetCode #1726 - Tuple with Same Product
use std::collections::HashMap;

fn tuple_same_product(nums: Vec<i32>) -> i32 {
    let mut freq: HashMap<i64, i32> = HashMap::new();
    let mut pairs = 0i64;
    for i in 0..nums.len() {
        for j in 0..i {
            let prod = nums[i] as i64 * nums[j] as i64;
            pairs += *freq.get(&prod).unwrap_or(&0) as i64;
            *freq.entry(prod).or_insert(0) += 1;
        }
    }
    (pairs * 8) as i32
}
fn main() {
    println!("{}", tuple_same_product(vec![2, 3, 4, 6]));
}
#[cfg(test)]
mod tests {
    use super::tuple_same_product;
    #[test]
    fn example_one() {
        assert_eq!(tuple_same_product(vec![2, 3, 4, 6]), 8);
    }
    #[test]
    fn example_two() {
        assert_eq!(tuple_same_product(vec![1, 2, 4, 5, 10]), 16);
    }
    #[test]
    fn example_three() {
        assert_eq!(tuple_same_product(vec![2, 3, 5, 7]), 0);
    }
}
