/// LeetCode #3209 - Number of Subarrays With AND Value of K
use std::collections::HashMap;

fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    let mut ans = 0i64;
    let mut pre: HashMap<i32, i64> = HashMap::new();
    for x in nums {
        let mut cur: HashMap<i32, i64> = HashMap::new();
        for (&y, &v) in &pre {
            *cur.entry(x & y).or_insert(0) += v;
        }
        *cur.entry(x).or_insert(0) += 1;
        ans += *cur.get(&k).unwrap_or(&0);
        pre = cur;
    }
    ans
}

fn main() {
    println!("{}", count_subarrays(vec![1, 1, 1], 1));
}

#[cfg(test)]
mod tests {
    use super::count_subarrays;

    #[test]
    fn example1() {
        assert_eq!(count_subarrays(vec![1, 1, 1], 1), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(count_subarrays(vec![1, 1, 2], 1), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(count_subarrays(vec![1, 2, 3], 2), 2);
    }
}
