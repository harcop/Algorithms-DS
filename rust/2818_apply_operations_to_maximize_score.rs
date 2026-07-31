/// LeetCode #2818 - Apply Operations to Maximize Score
use std::collections::HashSet;

fn prime_score(n: i32) -> i32 {
    let mut n = n;
    let mut i = 2;
    let mut factors = HashSet::new();
    while i * i <= n {
        while n % i == 0 {
            factors.insert(i);
            n /= i;
        }
        i += 1;
    }
    if n > 1 {
        factors.insert(n);
    }
    factors.len() as i32
}

fn qpow(mut a: i64, mut n: i64, modulo: i64) -> i64 {
    let mut ans = 1i64;
    while n > 0 {
        if n & 1 == 1 {
            ans = ans * a % modulo;
        }
        a = a * a % modulo;
        n >>= 1;
    }
    ans
}

fn maximum_score(nums: Vec<i32>, mut k: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = nums.len();
    let scores: Vec<i32> = nums.iter().map(|&x| prime_score(x)).collect();
    let mut left = vec![-1i32; n];
    let mut right = vec![n as i32; n];
    let mut stk: Vec<usize> = vec![];

    for i in 0..n {
        while let Some(&top) = stk.last() {
            if scores[top] < scores[i] {
                stk.pop();
            } else {
                break;
            }
        }
        if let Some(&top) = stk.last() {
            left[i] = top as i32;
        }
        stk.push(i);
    }

    stk.clear();
    for i in (0..n).rev() {
        while let Some(&top) = stk.last() {
            if scores[top] <= scores[i] {
                stk.pop();
            } else {
                break;
            }
        }
        if let Some(&top) = stk.last() {
            right[i] = top as i32;
        }
        stk.push(i);
    }

    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| nums[b].cmp(&nums[a]));

    let mut ans = 1i64;
    for &i in &order {
        let l = left[i] as i64;
        let r = right[i] as i64;
        let cnt = (i as i64 - l) * (r - i as i64);
        if cnt <= k as i64 {
            ans = ans * qpow(nums[i] as i64, cnt, MOD) % MOD;
            k -= cnt as i32;
        } else {
            ans = ans * qpow(nums[i] as i64, k as i64, MOD) % MOD;
            break;
        }
    }
    ans as i32
}

fn main() {
    println!("{}", maximum_score(vec![8, 3, 9, 3, 8], 2));
}

#[cfg(test)]
mod tests {
    use super::maximum_score;

    #[test]
    fn example_one() {
        assert_eq!(maximum_score(vec![8, 3, 9, 3, 8], 2), 81);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_score(vec![19, 12, 14, 6, 10, 18], 3), 4788);
    }
}
