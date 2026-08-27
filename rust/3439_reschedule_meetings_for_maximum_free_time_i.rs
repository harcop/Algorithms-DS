/// LeetCode #3439 - Reschedule Meetings for Maximum Free Time I
fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
    let n = end_time.len();
    let mut nums = Vec::with_capacity(n + 1);
    nums.push(start_time[0]);
    for i in 1..n {
        nums.push(start_time[i] - end_time[i - 1]);
    }
    nums.push(event_time - end_time[n - 1]);
    let k = k as usize;
    let mut ans = 0;
    let mut s = 0;
    for (i, &x) in nums.iter().enumerate() {
        s += x;
        if i >= k {
            ans = ans.max(s);
            s -= nums[i - k];
        }
    }
    ans
}

fn main() {
    println!("{}", max_free_time(5, 1, vec![1, 3], vec![2, 5]));
}

#[cfg(test)]
mod tests {
    use super::max_free_time;

    #[test]
    fn example1() {
        assert_eq!(max_free_time(5, 1, vec![1, 3], vec![2, 5]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(max_free_time(10, 1, vec![0, 2, 9], vec![1, 4, 10]), 6);
    }

    #[test]
    fn example3() {
        assert_eq!(
            max_free_time(5, 2, vec![0, 1, 2, 3, 4], vec![1, 2, 3, 4, 5]),
            0
        );
    }
}
