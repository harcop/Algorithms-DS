/// LeetCode #3169 - Count Days Without Meetings
fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
    meetings.sort_by_key(|m| m[0]);
    let mut ans = 0;
    let mut last = 0;
    for e in meetings {
        let st = e[0];
        let ed = e[1];
        if last < st {
            ans += st - last - 1;
        }
        last = last.max(ed);
    }
    ans + (days - last)
}

fn main() {
    println!(
        "{}",
        count_days(10, vec![vec![5, 7], vec![1, 3], vec![9, 10]])
    );
}

#[cfg(test)]
mod tests {
    use super::count_days;

    #[test]
    fn example1() {
        assert_eq!(
            count_days(10, vec![vec![5, 7], vec![1, 3], vec![9, 10]]),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(count_days(5, vec![vec![2, 4], vec![1, 3]]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(count_days(6, vec![vec![1, 6]]), 0);
    }
}
