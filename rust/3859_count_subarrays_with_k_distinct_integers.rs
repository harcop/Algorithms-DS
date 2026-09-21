/// LeetCode #3859 - Count Subarrays With K Distinct Integers
use std::collections::HashMap;

fn count_subarrays(nums: Vec<i32>, k: i32, m: i32) -> i64 {
    let f = |lim: usize| -> i64 {
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        let mut ans = 0i64;
        let mut l = 0usize;
        let mut t = 0i32;
        for &x in &nums {
            let e = cnt.entry(x).or_insert(0);
            *e += 1;
            if *e == m {
                t += 1;
            }
            while cnt.len() >= lim && t >= k {
                let y = nums[l];
                l += 1;
                let e = cnt.get_mut(&y).unwrap();
                *e -= 1;
                if *e == m - 1 {
                    t -= 1;
                }
                if *e == 0 {
                    cnt.remove(&y);
                }
            }
            ans += l as i64;
        }
        ans
    };
    f(k as usize) - f(k as usize + 1)
}

fn main() {
    println!("{}", count_subarrays(vec![1, 2, 1, 2, 2], 2, 2));
}

#[cfg(test)]
mod tests {
    use super::count_subarrays;

    #[test]
    fn example1() {
        assert_eq!(count_subarrays(vec![1, 2, 1, 2, 2], 2, 2), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(count_subarrays(vec![3, 1, 2, 4], 2, 1), 3);
    }
}
