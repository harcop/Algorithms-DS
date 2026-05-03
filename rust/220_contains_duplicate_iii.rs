/// LeetCode #220 - Contains Duplicate III
use std::collections::HashMap;

fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
    if t < 0 || k <= 0 {
        return false;
    }
    let k = k as usize;
    let t = t as i64;
    let w = t + 1;
    let mut buckets: HashMap<i64, i64> = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let x = num as i64;
        let id = if x < 0 {
            (x + 1) / w - 1
        } else {
            x / w
        };
        if buckets.contains_key(&id) {
            return true;
        }
        if let Some(&v) = buckets.get(&(id - 1)) {
            if (x - v).abs() <= t {
                return true;
            }
        }
        if let Some(&v) = buckets.get(&(id + 1)) {
            if (x - v).abs() <= t {
                return true;
            }
        }
        buckets.insert(id, x);
        if i >= k {
            let old = nums[i - k] as i64;
            let oid = if old < 0 {
                (old + 1) / w - 1
            } else {
                old / w
            };
            buckets.remove(&oid);
        }
    }
    false
}

fn main() {
    println!("{}", contains_nearby_almost_duplicate(vec![1, 2, 3, 1], 3, 0));
}

#[cfg(test)]
mod tests {
    use super::contains_nearby_almost_duplicate;

    #[test]
    fn example_one() {
        assert!(contains_nearby_almost_duplicate(vec![1, 2, 3, 1], 3, 0));
    }

    #[test]
    fn example_two() {
        assert!(contains_nearby_almost_duplicate(vec![4, 1, 2, 3], 2, 1));
    }
}
