/// LeetCode #2799 - Count Complete Subarrays in an Array
use std::collections::HashMap;

fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
    let mut total: HashMap<i32, i32> = HashMap::new();
    for &x in &nums {
        *total.entry(x).or_insert(0) += 1;
    }
    let cnt = total.len();
    let n = nums.len();
    let mut ans = 0;
    let mut i = 0;
    let mut d: HashMap<i32, i32> = HashMap::new();
    for j in 0..n {
        *d.entry(nums[j]).or_insert(0) += 1;
        while d.len() == cnt {
            ans += (n - j) as i32;
            let e = d.get_mut(&nums[i]).unwrap();
            *e -= 1;
            if *e == 0 {
                d.remove(&nums[i]);
            }
            i += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", count_complete_subarrays(vec![1, 3, 1, 2, 2]));
}

#[cfg(test)]
mod tests {
    use super::count_complete_subarrays;

    #[test]
    fn example_one() {
        assert_eq!(count_complete_subarrays(vec![1, 3, 1, 2, 2]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_complete_subarrays(vec![5, 5, 5, 5]), 10);
    }
}
