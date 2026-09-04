/// LeetCode #1454 - Active Users (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn parse_ymd(s: &str) -> (i32, u32, u32) {
    let mut p = s.split('-');
    (
        p.next().unwrap().parse().unwrap(),
        p.next().unwrap().parse().unwrap(),
        p.next().unwrap().parse().unwrap(),
    )
}

fn days_from_civil(y: i32, m: u32, d: u32) -> i32 {
    let mut y = y;
    if m <= 2 {
        y -= 1;
    }
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = (y - era * 400) as u32;
    let mp = if m > 2 { m - 3 } else { m + 9 };
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe as i32 - 719468
}

fn day_num(s: &str) -> i32 {
    let (y, m, d) = parse_ymd(s);
    days_from_civil(y, m, d)
}

fn active_users(accounts: Vec<(i32, String)>, logins: Vec<(i32, String)>) -> Vec<(i32, String)> {
    let names: HashMap<i32, String> = accounts.into_iter().collect();
    let mut days: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (id, date) in logins {
        days.entry(id).or_default().insert(day_num(&date));
    }
    let mut ans = Vec::new();
    for (id, set) in days {
        let mut v: Vec<i32> = set.into_iter().collect();
        v.sort();
        let mut run = 1;
        let mut ok = false;
        for i in 1..v.len() {
            if v[i] == v[i - 1] + 1 {
                run += 1;
            } else {
                run = 1;
            }
            if run >= 5 {
                ok = true;
                break;
            }
        }
        if ok {
            ans.push((id, names[&id].clone()));
        }
    }
    ans.sort_by_key(|r| r.0);
    ans
}

fn main() {
    println!("{:?}", active_users(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::active_users;

    #[test]
    fn example() {
        let accounts = vec![(1, "Winston".into()), (7, "Jonathan".into())];
        let logins = vec![
            (7, "2020-05-30".into()),
            (1, "2020-05-30".into()),
            (7, "2020-05-31".into()),
            (7, "2020-06-01".into()),
            (7, "2020-06-02".into()),
            (7, "2020-06-02".into()),
            (7, "2020-06-03".into()),
            (1, "2020-06-07".into()),
            (7, "2020-06-10".into()),
        ];
        assert_eq!(active_users(accounts, logins), vec![(7, "Jonathan".into())]);
    }
}
