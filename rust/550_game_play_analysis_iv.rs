/// LeetCode #550 - Game Play Analysis IV (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

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

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

fn login_fraction(activity: Vec<(i32, i32, String, i32)>) -> f64 {
    let mut first: HashMap<i32, String> = HashMap::new();
    let mut dates: HashSet<(i32, i32)> = HashSet::new();
    for (player_id, _, event_date, _) in activity {
        dates.insert((player_id, date_num(&event_date)));
        first
            .entry(player_id)
            .and_modify(|d| {
                if event_date < *d {
                    *d = event_date.clone();
                }
            })
            .or_insert(event_date);
    }
    if first.is_empty() {
        return 0.0;
    }
    let n = first.len() as f64;
    let ok = first
        .iter()
        .filter(|(pid, d)| dates.contains(&(**pid, date_num(d) + 1)))
        .count() as f64;
    round2(ok / n)
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::login_fraction;

    #[test]
    fn example() {
        let activity = vec![
            (1, 2, "2016-03-01".into(), 5),
            (1, 2, "2016-03-02".into(), 6),
            (2, 3, "2017-06-25".into(), 1),
            (3, 1, "2016-03-02".into(), 0),
            (3, 4, "2018-07-03".into(), 5),
        ];
        assert!((login_fraction(activity) - 0.33).abs() < 1e-9);
    }
}
