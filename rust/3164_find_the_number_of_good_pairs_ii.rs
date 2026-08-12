/// LeetCode #3164 - Find the Number of Good Pairs II
use std::collections::HashMap;

fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    let mut cnt1: HashMap<i32, i64> = HashMap::new();
    for x in nums1 {
        if x % k == 0 {
            *cnt1.entry(x / k).or_insert(0) += 1;
        }
    }
    if cnt1.is_empty() {
        return 0;
    }
    let mx = *cnt1.keys().max().unwrap();
    let mut cnt2: HashMap<i32, i64> = HashMap::new();
    for y in nums2 {
        *cnt2.entry(y).or_insert(0) += 1;
    }
    let mut ans = 0i64;
    for (x, v) in cnt2 {
        let mut y = x;
        let mut s = 0i64;
        while y <= mx {
            if let Some(&c) = cnt1.get(&y) {
                s += c;
            }
            y += x;
        }
        ans += s * v;
    }
    ans
}

fn main() {
    println!("{}", number_of_pairs(vec![1, 3, 4], vec![1, 3, 4], 1));
}

#[cfg(test)]
mod tests {
    use super::number_of_pairs;

    #[test]
    fn example1() {
        assert_eq!(number_of_pairs(vec![1, 3, 4], vec![1, 3, 4], 1), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(number_of_pairs(vec![1, 2, 4, 12], vec![2, 4], 3), 2);
    }
}
