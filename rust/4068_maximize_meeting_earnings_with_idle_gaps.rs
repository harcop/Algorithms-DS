/// LeetCode #4068 - Maximize Meeting Earnings with Idle Gaps
fn max_earnings(mut meetings: Vec<Vec<i32>>) -> i64 {
    meetings.sort_unstable_by_key(|m| m[1]);
    let n = meetings.len();
    let mut pre_max = vec![i64::MIN / 4; n + 1];
    let mut ans = 0i64;
    for i in 0..n {
        let start = meetings[i][0];
        let end = meetings[i][1];
        let revenue = meetings[i][2] as i64;
        let mut val = revenue;
        if start >= meetings[0][1] {
            let j = meetings[..i].partition_point(|m| m[1] <= start);
            val += pre_max[j] + start as i64;
        }
        ans = ans.max(val);
        pre_max[i + 1] = pre_max[i].max(val - end as i64);
    }
    ans
}

fn main() {
    println!("{}", max_earnings(vec![vec![2, 5, 4], vec![6, 8, 3]]));
}

#[cfg(test)]
mod tests {
    use super::max_earnings;

    #[test]
    fn example1() {
        assert_eq!(max_earnings(vec![vec![2, 5, 4], vec![6, 8, 3]]), 8);
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_earnings(vec![vec![3, 5, 4], vec![4, 7, 8], vec![8, 10, 3]]),
            12
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            max_earnings(vec![vec![1, 2, 2], vec![4, 5, 2], vec![7, 9, 3]]),
            11
        );
    }
}
