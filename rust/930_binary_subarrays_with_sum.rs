/// LeetCode #930 - Binary Subarrays With Sum
use std::collections::HashMap;

fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
    let mut prefix = 0i32;
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    cnt.insert(0, 1);
    let mut ans = 0i32;
    for &x in &nums {
        prefix += x;
        if let Some(&c) = cnt.get(&(prefix - goal)) {
            ans += c;
        }
        *cnt.entry(prefix).or_insert(0) += 1;
    }
    ans
}

fn main() {
    println!("{}", num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2));
}

#[cfg(test)]
mod tests {
    use super::num_subarrays_with_sum;

    #[test]
    fn example_one() {
        assert_eq!(num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_subarrays_with_sum(vec![0, 0, 0, 0, 0], 0), 15);
    }
}
