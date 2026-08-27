/// LeetCode #3440 - Reschedule Meetings for Maximum Free Time II
fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
    let n = start_time.len();
    let mut pre = vec![0; n];
    let mut suf = vec![0; n];
    pre[0] = start_time[0];
    suf[n - 1] = event_time - end_time[n - 1];
    for i in 1..n {
        pre[i] = pre[i - 1].max(start_time[i] - end_time[i - 1]);
    }
    for i in (0..n - 1).rev() {
        suf[i] = suf[i + 1].max(start_time[i + 1] - end_time[i]);
    }
    let mut ans = 0;
    for i in 0..n {
        let l = if i == 0 { 0 } else { end_time[i - 1] };
        let r = if i == n - 1 {
            event_time
        } else {
            start_time[i + 1]
        };
        let w = end_time[i] - start_time[i];
        ans = ans.max(r - l - w);
        if i > 0 && pre[i - 1] >= w {
            ans = ans.max(r - l);
        } else if i + 1 < n && suf[i + 1] >= w {
            ans = ans.max(r - l);
        }
    }
    ans
}

fn main() {
    println!("{}", max_free_time(5, vec![1, 3], vec![2, 5]));
}

#[cfg(test)]
mod tests {
    use super::max_free_time;

    #[test]
    fn example1() {
        assert_eq!(max_free_time(5, vec![1, 3], vec![2, 5]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(max_free_time(10, vec![0, 7, 9], vec![1, 8, 10]), 7);
    }

    #[test]
    fn example3() {
        assert_eq!(max_free_time(10, vec![0, 3, 7, 9], vec![1, 4, 8, 10]), 6);
    }

    #[test]
    fn example4() {
        assert_eq!(
            max_free_time(5, vec![0, 1, 2, 3, 4], vec![1, 2, 3, 4, 5]),
            0
        );
    }
}
