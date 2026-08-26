/// LeetCode #3430 - Maximum and Minimum Sums of at Most Size K Subarrays
use std::collections::VecDeque;

fn min_max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;
    let mut total = 0i64;
    let mut max_stack: VecDeque<(usize, i64, i64)> = VecDeque::new();
    let mut min_stack: VecDeque<(usize, i64, i64)> = VecDeque::new();
    let mut sub_max = 0i64;
    let mut sub_min = 0i64;
    for (end, &x) in nums.iter().enumerate() {
        let num = x as i64;
        let start = end.saturating_sub(k - 1);
        if start > 0 {
            max_stack[0].2 -= 1;
            sub_max -= max_stack[0].1;
            if max_stack[0].0 < start {
                max_stack.pop_front();
            }
            min_stack[0].2 -= 1;
            sub_min -= min_stack[0].1;
            if min_stack[0].0 < start {
                min_stack.pop_front();
            }
        }
        let mut max_shares = 1i64;
        sub_max += num;
        while let Some(&(_, prev_num, prev_shares)) = max_stack.back() {
            if prev_num > num {
                break;
            }
            max_stack.pop_back();
            max_shares += prev_shares;
            sub_max += (num - prev_num) * prev_shares;
        }
        max_stack.push_back((end, num, max_shares));
        let mut min_shares = 1i64;
        sub_min += num;
        while let Some(&(_, prev_num, prev_shares)) = min_stack.back() {
            if prev_num < num {
                break;
            }
            min_stack.pop_back();
            min_shares += prev_shares;
            sub_min += (num - prev_num) * prev_shares;
        }
        min_stack.push_back((end, num, min_shares));
        total += sub_max + sub_min;
    }
    total
}

fn main() {
    println!("{}", min_max_subarray_sum(vec![1, 2, 3], 2));
}

#[cfg(test)]
mod tests {
    use super::min_max_subarray_sum;

    #[test]
    fn example1() {
        assert_eq!(min_max_subarray_sum(vec![1, 2, 3], 2), 20);
    }

    #[test]
    fn example2() {
        assert_eq!(min_max_subarray_sum(vec![1, -3, 1], 2), -6);
    }
}
