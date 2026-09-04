/// LeetCode #1890 - The Latest Login in 2020 (SQL; Rust analogue)
use std::collections::HashMap;

fn latest_login_in_2020(logins: Vec<(i32, String)>) -> Vec<(i32, String)> {
    let mut last: HashMap<i32, String> = HashMap::new();
    for (user_id, time_stamp) in logins {
        if time_stamp.starts_with("2020-") {
            last.entry(user_id)
                .and_modify(|t| {
                    if time_stamp > *t {
                        *t = time_stamp.clone();
                    }
                })
                .or_insert(time_stamp);
        }
    }
    let mut ans: Vec<(i32, String)> = last.into_iter().collect();
    ans.sort_by_key(|t| t.0);
    ans
}

fn main() {
    let logins = vec![
        (6, "2020-06-30 15:06:07".into()),
        (6, "2021-04-21 14:06:06".into()),
        (6, "2019-03-07 00:18:15".into()),
        (8, "2020-02-01 05:10:53".into()),
        (8, "2020-12-30 00:46:50".into()),
        (2, "2020-01-16 02:49:50".into()),
        (2, "2019-08-25 07:59:08".into()),
        (14, "2019-07-14 09:00:00".into()),
        (14, "2021-01-06 11:59:59".into()),
    ];
    println!("{:?}", latest_login_in_2020(logins));
}

#[cfg(test)]
mod tests {
    use super::latest_login_in_2020;

    #[test]
    fn example_one() {
        let logins = vec![
            (6, "2020-06-30 15:06:07".into()),
            (6, "2021-04-21 14:06:06".into()),
            (6, "2019-03-07 00:18:15".into()),
            (8, "2020-02-01 05:10:53".into()),
            (8, "2020-12-30 00:46:50".into()),
            (2, "2020-01-16 02:49:50".into()),
            (2, "2019-08-25 07:59:08".into()),
            (14, "2019-07-14 09:00:00".into()),
            (14, "2021-01-06 11:59:59".into()),
        ];
        assert_eq!(
            latest_login_in_2020(logins),
            vec![
                (2, "2020-01-16 02:49:50".into()),
                (6, "2020-06-30 15:06:07".into()),
                (8, "2020-12-30 00:46:50".into()),
            ]
        );
    }
}
