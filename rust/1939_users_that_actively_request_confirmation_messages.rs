/// LeetCode #1939 - Users That Actively Request Confirmation Messages (SQL; Rust analogue)
use std::collections::HashMap;

fn days_from_civil(mut y: i64, m: i64, d: i64) -> i64 {
    y -= i64::from(m <= 2);
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let doy = (153 * (m + if m > 2 { -3 } else { 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn parse_dt(s: &str) -> i64 {
    let b: Vec<i64> = s
        .split(|c: char| !c.is_ascii_digit())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    days_from_civil(b[0], b[1], b[2]) * 86400 + b[3] * 3600 + b[4] * 60 + b[5]
}

fn users_actively_request(
    _signups: Vec<(i32, String)>,
    confirmations: Vec<(i32, String, String)>,
) -> Vec<i32> {
    let mut times: HashMap<i32, Vec<i64>> = HashMap::new();
    for (user_id, ts, _) in confirmations {
        times.entry(user_id).or_default().push(parse_dt(&ts));
    }
    let mut ans = Vec::new();
    for (user_id, mut t) in times {
        t.sort();
        if t.windows(2).any(|w| w[1] - w[0] <= 24 * 60 * 60) {
            ans.push(user_id);
        }
    }
    ans.sort();
    ans
}

fn main() {
    println!("{:?}", users_actively_request(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::users_actively_request;

    #[test]
    fn example_one() {
        let signups = vec![
            (3, "2020-03-21 10:16:13".into()),
            (7, "2020-01-04 13:57:59".into()),
            (2, "2020-07-29 23:09:44".into()),
            (6, "2020-12-09 10:39:37".into()),
        ];
        let confirmations = vec![
            (3, "2021-01-06 03:30:46".into(), "timeout".into()),
            (3, "2021-01-06 03:37:45".into(), "timeout".into()),
            (7, "2021-06-12 11:57:29".into(), "confirmed".into()),
            (7, "2021-06-13 11:57:30".into(), "confirmed".into()),
            (2, "2021-01-22 00:00:00".into(), "confirmed".into()),
            (2, "2021-01-23 00:00:00".into(), "timeout".into()),
            (6, "2021-10-23 14:14:14".into(), "confirmed".into()),
            (6, "2021-10-24 14:14:13".into(), "timeout".into()),
        ];
        assert_eq!(users_actively_request(signups, confirmations), vec![2, 3, 6]);
    }
}
