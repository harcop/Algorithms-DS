/// LeetCode #3420 - Count Non-Decreasing Subarrays After K Operations
use std::collections::VecDeque;

fn count_non_decreasing_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let k = k as i64;
    let mut ans = 0i64;
    let mut cost = 0i64;
    let mut dq: VecDeque<(i32, i32)> = VecDeque::new();
    let mut j = n as i32 - 1;
    for i in (0..n).rev() {
        let num = nums[i];
        let mut count = 1i32;
        while let Some(&(next_num, next_count)) = dq.back() {
            if next_num >= num {
                break;
            }
            dq.pop_back();
            count += next_count;
            cost += (num - next_num) as i64 * next_count as i64;
        }
        dq.push_back((num, count));
        while cost > k {
            let (rightmost_num, rightmost_count) = dq.pop_front().unwrap();
            cost -= (rightmost_num - nums[j as usize]) as i64;
            j -= 1;
            if rightmost_count > 1 {
                dq.push_front((rightmost_num, rightmost_count - 1));
            }
        }
        ans += (j - i as i32 + 1) as i64;
    }
    ans
}

fn main() {
    println!(
        "{}",
        count_non_decreasing_subarrays(vec![6, 3, 1, 2, 4, 4], 7)
    );
}

#[cfg(test)]
mod tests {
    use super::count_non_decreasing_subarrays;

    #[test]
    fn example1() {
        assert_eq!(
            count_non_decreasing_subarrays(vec![6, 3, 1, 2, 4, 4], 7),
            17
        );
    }

    #[test]
    fn example2() {
        assert_eq!(count_non_decreasing_subarrays(vec![6, 3, 1, 3, 6], 4), 12);
    }
}
