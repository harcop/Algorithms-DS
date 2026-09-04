/// LeetCode #1709 - Biggest Window Between Visits (SQL; Rust analogue)
use std::collections::BTreeMap;

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

fn biggest_window(visits: Vec<(i32, String)>) -> Vec<(i32, i32)> {
    let today = days_from_civil(2021, 1, 1);
    let mut by_user: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    for (uid, date) in visits {
        by_user.entry(uid).or_default().push(day_num(&date));
    }
    by_user
        .into_iter()
        .map(|(uid, mut days)| {
            days.sort();
            days.push(today);
            let mut max_w = 0;
            for i in 1..days.len() {
                max_w = max_w.max(days[i] - days[i - 1]);
            }
            (uid, max_w)
        })
        .collect()
}

fn main() {
    println!("{:?}", biggest_window(vec![]));
}

#[cfg(test)]
mod tests {
    use super::biggest_window;

    #[test]
    fn example() {
        let visits = vec![
            (1, "2020-11-28".into()),
            (1, "2020-10-20".into()),
            (1, "2020-12-3".into()),
            (2, "2020-10-5".into()),
            (2, "2020-12-9".into()),
            (3, "2020-11-11".into()),
        ];
        assert_eq!(biggest_window(visits), vec![(1, 39), (2, 65), (3, 51)]);
    }
}
