/// LeetCode #1107 - New Users Daily Count (SQL; Rust analogue)

fn date_num(s: &str) -> i32 {
    let p: Vec<i32> = s
        .split(|c: char| !c.is_ascii_digit())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    let (y, m, d) = (p[0], p[1], p[2]);
    let a = (14 - m) / 12;
    let y = y + 4800 - a;
    let m = m + 12 * a - 3;
    d + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400
}

use std::collections::HashMap;

fn new_users_daily_count(traffic: Vec<(i32, String, String)>) -> Vec<(String, i32)> {
    let today = date_num("2019-06-30");
    let mut first: HashMap<i32, String> = HashMap::new();
    for (uid, activity, date) in traffic {
        if activity != "login" {
            continue;
        }
        first
            .entry(uid)
            .and_modify(|d| {
                if date < *d {
                    *d = date.clone();
                }
            })
            .or_insert(date);
    }
    let mut cnt: HashMap<String, i32> = HashMap::new();
    for (_, d) in first {
        if today - date_num(&d) <= 90 {
            *cnt.entry(d).or_insert(0) += 1;
        }
    }
    let mut ans: Vec<(String, i32)> = cnt.into_iter().collect();
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::new_users_daily_count;

    #[test]
    fn example() {
        let traffic = vec![
            (1, "login".into(), "2019-05-01".into()),
            (1, "homepage".into(), "2019-05-01".into()),
            (1, "logout".into(), "2019-05-01".into()),
            (2, "login".into(), "2019-06-21".into()),
            (2, "logout".into(), "2019-06-21".into()),
            (3, "login".into(), "2019-01-01".into()),
            (3, "jobs".into(), "2019-01-01".into()),
            (3, "logout".into(), "2019-01-01".into()),
            (4, "login".into(), "2019-06-21".into()),
            (4, "groups".into(), "2019-06-21".into()),
            (4, "logout".into(), "2019-06-21".into()),
            (5, "login".into(), "2019-03-01".into()),
            (5, "logout".into(), "2019-03-01".into()),
            (5, "login".into(), "2019-06-21".into()),
            (5, "logout".into(), "2019-06-21".into()),
        ];
        assert_eq!(
            new_users_daily_count(traffic),
            vec![("2019-05-01".into(), 1), ("2019-06-21".into(), 2)]
        );
    }
}
