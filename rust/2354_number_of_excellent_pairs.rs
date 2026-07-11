/// LeetCode #2354 - Number of Excellent Pairs
use std::collections::HashSet;

fn count_excellent_pairs(nums: Vec<i32>, k: i32) -> i64 {
    let s: HashSet<i32> = nums.into_iter().collect();
    let mut cnt = [0i64; 32];
    for &v in &s {
        cnt[v.count_ones() as usize] += 1;
    }
    let mut ans = 0i64;
    for &v in &s {
        let t = v.count_ones() as i32;
        for i in 0..32 {
            if t + i >= k {
                ans += cnt[i as usize];
            }
        }
    }
    ans
}

fn main() {
    println!("{}", count_excellent_pairs(vec![1, 2, 3, 1], 3));
}

#[cfg(test)]
mod tests {
    use super::count_excellent_pairs;

    #[test]
    fn example_one() {
        assert_eq!(count_excellent_pairs(vec![1, 2, 3, 1], 3), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_excellent_pairs(vec![5, 1, 1], 10), 0);
    }
}
