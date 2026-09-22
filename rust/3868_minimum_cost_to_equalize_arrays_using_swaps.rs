/// LeetCode #3868 - Minimum Cost to Equalize Arrays Using Swaps
use std::collections::HashMap;

fn min_cost(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut cnt2: HashMap<i32, i32> = HashMap::new();
    for x in nums2 {
        *cnt2.entry(x).or_insert(0) += 1;
    }
    let mut cnt1: HashMap<i32, i32> = HashMap::new();
    for x in nums1 {
        if let Some(c) = cnt2.get_mut(&x) {
            if *c > 0 {
                *c -= 1;
                continue;
            }
        }
        *cnt1.entry(x).or_insert(0) += 1;
    }
    let mut ans = 0;
    for &v in cnt1.values() {
        if v % 2 == 1 {
            return -1;
        }
        ans += v / 2;
    }
    for &v in cnt2.values() {
        if v % 2 == 1 {
            return -1;
        }
    }
    ans
}

fn main() {
    println!("{}", min_cost(vec![10, 20], vec![20, 10]));
}

#[cfg(test)]
mod tests {
    use super::min_cost;

    #[test]
    fn example1() {
        assert_eq!(min_cost(vec![10, 20], vec![20, 10]), 0);
    }

    #[test]
    fn example2() {
        assert_eq!(min_cost(vec![10, 10], vec![20, 20]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(min_cost(vec![10, 20], vec![30, 40]), -1);
    }
}
