/// LeetCode #3589 - Count Prime-Gap Balanced Subarrays
use std::collections::VecDeque;

fn sieve(n: usize) -> Vec<bool> {
    let mut is_prime = vec![true; n + 1];
    if n >= 0 {
        is_prime[0] = false;
    }
    if n >= 1 {
        is_prime[1] = false;
    }
    let mut i = 2usize;
    while i * i <= n {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    is_prime
}

fn prime_subarray(nums: Vec<i32>, k: i32) -> i32 {
    let is_prime = sieve(50_000);
    let n = nums.len();
    let mut minq: VecDeque<(i32, usize)> = VecDeque::new();
    let mut maxq: VecDeque<(i32, usize)> = VecDeque::new();
    let mut left = 0usize;
    let mut p1 = -1i32;
    let mut p2 = -1i32;
    let mut ans = 0i64;
    for r in 0..n {
        let x = nums[r];
        if x as usize <= 50_000 && is_prime[x as usize] {
            while minq.back().map(|&(v, _)| v > x).unwrap_or(false) {
                minq.pop_back();
            }
            minq.push_back((x, r));
            while maxq.back().map(|&(v, _)| v < x).unwrap_or(false) {
                maxq.pop_back();
            }
            maxq.push_back((x, r));
            p2 = p1;
            p1 = r as i32;
        }
        loop {
            while minq.front().map(|&(_, i)| i < left).unwrap_or(false) {
                minq.pop_front();
            }
            while maxq.front().map(|&(_, i)| i < left).unwrap_or(false) {
                maxq.pop_front();
            }
            if minq.is_empty() || maxq.is_empty() {
                break;
            }
            if maxq.front().unwrap().0 - minq.front().unwrap().0 <= k {
                break;
            }
            left += 1;
        }
        if p2 >= left as i32 {
            ans += (p2 - left as i32 + 1) as i64;
        }
    }
    ans as i32
}

fn main() {
    println!("{}", prime_subarray(vec![1, 2, 3], 1));
}

#[cfg(test)]
mod tests {
    use super::prime_subarray;

    #[test]
    fn example1() {
        assert_eq!(prime_subarray(vec![1, 2, 3], 1), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(prime_subarray(vec![2, 3, 5, 7], 3), 4);
    }
}
