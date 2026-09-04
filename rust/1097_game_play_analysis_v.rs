/// LeetCode #1097 - Game Play Analysis V (SQL; Rust analogue)

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

use std::collections::{HashMap, HashSet};

fn game_play_analysis_v(activity: Vec<(i32, i32, String, i32)>) -> Vec<(String, i32, f64)> {
    let mut first: HashMap<i32, String> = HashMap::new();
    let mut dates: HashSet<(i32, i32)> = HashSet::new();
    for (pid, _, d, _) in &activity {
        dates.insert((*pid, date_num(d)));
        first
            .entry(*pid)
            .and_modify(|x| {
                if d < x {
                    *x = d.clone();
                }
            })
            .or_insert_with(|| d.clone());
    }
    let mut by_day: HashMap<String, (i32, i32)> = HashMap::new();
    for (pid, d) in first {
        let e = by_day.entry(d.clone()).or_insert((0, 0));
        e.0 += 1;
        if dates.contains(&(pid, date_num(&d) + 1)) {
            e.1 += 1;
        }
    }
    let mut ans: Vec<(String, i32, f64)> = by_day
        .into_iter()
        .map(|(d, (n, ret))| (d, n, round2(ret as f64 / n as f64)))
        .collect();
    ans.sort_by(|a, b| a.0.cmp(&b.0));
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::game_play_analysis_v;

    #[test]
    fn example() {
        let activity = vec![
            (1, 2, "2016-03-01".into(), 5),
            (1, 2, "2016-03-02".into(), 6),
            (2, 3, "2017-06-25".into(), 1),
            (3, 1, "2016-03-01".into(), 0),
            (3, 4, "2016-07-03".into(), 5),
        ];
        let got = game_play_analysis_v(activity);
        assert_eq!(got[0].0, "2016-03-01");
        assert_eq!(got[0].1, 2);
        assert!((got[0].2 - 0.50).abs() < 1e-9);
        assert_eq!(got[1].0, "2017-06-25");
        assert_eq!(got[1].1, 1);
        assert!((got[1].2 - 0.00).abs() < 1e-9);
    }
}
