/// LeetCode #3755 - Find Maximum Balanced XOR Subarray Length
use std::collections::HashMap;

fn max_balanced_subarray(nums: Vec<i32>) -> i32 {
    let mut d: HashMap<(i32, i32), i32> = HashMap::new();
    d.insert((0, 0), -1);
    let mut a = 0i32;
    let mut b = 0i32;
    let mut ans = 0i32;
    for (i, x) in nums.into_iter().enumerate() {
        a ^= x;
        b += if x % 2 == 0 { 1 } else { -1 };
        if let Some(&prev) = d.get(&(a, b)) {
            ans = ans.max(i as i32 - prev);
        } else {
            d.insert((a, b), i as i32);
        }
    }
    ans
}

fn main() {
    println!("{}", max_balanced_subarray(vec![3, 1, 3, 2, 0]));
}

#[cfg(test)]
mod tests {
    use super::max_balanced_subarray;

    #[test]
    fn example1() {
        assert_eq!(max_balanced_subarray(vec![3, 1, 3, 2, 0]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(max_balanced_subarray(vec![3, 2, 8, 5, 4, 14, 9, 15]), 8);
    }

    #[test]
    fn example3() {
        assert_eq!(max_balanced_subarray(vec![0]), 0);
    }
}
