/// LeetCode #3956 - Maximum Sum of M Non-Overlapping Subarrays I
use std::collections::VecDeque;

fn max_sum(nums: Vec<i32>, m: i32, l: i32, r: i32) -> i64 {
    let n = nums.len();
    let m = m as usize;
    let l = l as usize;
    let r = r as usize;
    let mut s = vec![0i64; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + nums[i] as i64;
    }
    let neg = i64::MIN / 4;
    let mut prev = vec![0i64; n + 1];
    let mut ans = neg;
    for _t in 1..=m {
        let mut cur = vec![neg; n + 1];
        let mut dq: VecDeque<usize> = VecDeque::new();
        for i in 1..=n {
            if i >= l {
                let j = i - l;
                if prev[j] > neg / 2 {
                    let val = prev[j] - s[j];
                    while dq
                        .back()
                        .map(|&b| prev[b] - s[b] <= val)
                        .unwrap_or(false)
                    {
                        dq.pop_back();
                    }
                    dq.push_back(j);
                }
            }
            let left = i as isize - r as isize;
            while dq.front().map(|&j| (j as isize) < left).unwrap_or(false) {
                dq.pop_front();
            }
            cur[i] = cur[i - 1];
            if let Some(&j) = dq.front() {
                cur[i] = cur[i].max(s[i] + prev[j] - s[j]);
            }
        }
        ans = ans.max(cur[n]);
        prev = cur;
    }
    ans
}

fn main() {
    println!("{}", max_sum(vec![4, 1, -5, 2], 2, 1, 3));
}

#[cfg(test)]
mod tests {
    use super::max_sum;

    #[test]
    fn example1() {
        assert_eq!(max_sum(vec![4, 1, -5, 2], 2, 1, 3), 7);
    }

    #[test]
    fn example2() {
        assert_eq!(max_sum(vec![1, 0, 3, 4], 2, 1, 2), 8);
    }

    #[test]
    fn example3() {
        assert_eq!(max_sum(vec![-1, 7, -4], 1, 2, 3), 6);
    }

    #[test]
    fn example4() {
        assert_eq!(max_sum(vec![-3, -4, -1], 2, 1, 2), -1);
    }
}
