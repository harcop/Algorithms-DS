/// LeetCode #1577 - Number Of Ways Where Square Of Number Is Equal To Product Of Two Numbers
use std::collections::HashMap;

fn num_triplets(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
    let k2 = k as i64 * k as i64;

    fn count(nums: &[i32], k2: i64) -> HashMap<i64, i32> {
        let mut cnt = HashMap::new();
        let n = nums.len();
        for j in 0..n {
            for l in 0..n {
                if j == l {
                    continue;
                }
                *cnt.entry(nums[j] as i64 * nums[l] as i64 * k2).or_default() += 1;
            }
        }
        cnt
    }

    fn cal(nums: &[i32], cnt: &HashMap<i64, i32>, k2: i64) -> i32 {
        nums.iter()
            .map(|&x| cnt.get(&(x as i64 * x as i64 * k2)).copied().unwrap_or(0))
            .sum()
    }

    let c1 = count(&nums1, k2);
    let c2 = count(&nums2, k2);
    cal(&nums1, &c2, k2) + cal(&nums2, &c1, k2)
}

fn main() {
    println!("{}", num_triplets(vec![1, 2, 4, 12], vec![2, 4], 3));
}

#[cfg(test)]
mod tests {
    use super::num_triplets;

    #[test]
    fn example_one() {
        assert_eq!(num_triplets(vec![1, 2, 4, 12], vec![2, 4], 3), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_triplets(vec![7], vec![7], 1), 0);
    }
}
