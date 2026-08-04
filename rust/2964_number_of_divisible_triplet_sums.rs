/// LeetCode #2964 - Number of Divisible Triplet Sums (Premium)
use std::collections::HashMap;

fn divisible_triplet_count(nums: Vec<i32>, d: i32) -> i32 {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let mut ans = 0;
    let n = nums.len();
    for j in 0..n {
        for k in (j + 1)..n {
            let x = (d - (nums[j] + nums[k]) % d) % d;
            ans += *cnt.get(&x).unwrap_or(&0);
        }
        *cnt.entry(nums[j] % d).or_insert(0) += 1;
    }
    ans
}

fn main() {
    println!("{}", divisible_triplet_count(vec![3, 3, 4, 7, 8], 5));
}

#[cfg(test)]
mod tests {
    use super::divisible_triplet_count;

    #[test]
    fn example_one() {
        assert_eq!(divisible_triplet_count(vec![3, 3, 4, 7, 8], 5), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(divisible_triplet_count(vec![3, 3, 3, 3], 3), 4);
    }

    #[test]
    fn example_three() {
        assert_eq!(divisible_triplet_count(vec![3, 3, 3, 3], 6), 0);
    }
}
