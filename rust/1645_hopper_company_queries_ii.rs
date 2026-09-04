/// LeetCode #1645 - Hopper Company Queries II (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn parse_ymd(s: &str) -> (i32, u32, u32) {
    let mut p = s.split('-');
    (
        p.next().unwrap().parse().unwrap(),
        p.next().unwrap().parse().unwrap(),
        p.next().unwrap().parse().unwrap(),
    )
}

fn hopper_queries_ii(
    drivers: Vec<(i32, String)>,
    rides: Vec<(i32, i32, String)>,
    accepted: Vec<(i32, i32, i32, i32)>,
) -> Vec<(i32, f64)> {
    let ride_driver: HashMap<i32, i32> = accepted.into_iter().map(|(id, d, _, _)| (id, d)).collect();
    let mut ans = Vec::new();
    for month in 1..=12 {
        let active = drivers
            .iter()
            .filter(|(_, join)| {
                let (y, m, _) = parse_ymd(join);
                y < 2020 || (y == 2020 && m <= month as u32)
            })
            .count() as f64;
        let working: HashSet<i32> = rides
            .iter()
            .filter_map(|(id, _, req)| {
                let (y, m, _) = parse_ymd(req);
                if y == 2020 && m == month as u32 {
                    ride_driver.get(id).copied()
                } else {
                    None
                }
            })
            .collect();
        let pct = if active == 0.0 {
            0.0
        } else {
            (working.len() as f64 / active * 10000.0).round() / 100.0
        };
        ans.push((month, pct));
    }
    ans
}

fn main() {
    println!("{:?}", hopper_queries_ii(vec![], vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::hopper_queries_ii;

    #[test]
    fn example() {
        let drivers = vec![
            (10, "2019-12-10".into()),
            (8, "2020-1-13".into()),
            (5, "2020-2-16".into()),
            (7, "2020-3-8".into()),
            (4, "2020-5-17".into()),
            (1, "2020-10-24".into()),
            (6, "2021-1-5".into()),
        ];
        let rides = vec![
            (6, 75, "2019-12-9".into()),
            (1, 54, "2020-2-9".into()),
            (10, 63, "2020-3-4".into()),
            (19, 39, "2020-4-6".into()),
            (3, 41, "2020-6-3".into()),
            (13, 52, "2020-6-22".into()),
            (7, 69, "2020-7-16".into()),
            (17, 70, "2020-8-25".into()),
            (20, 81, "2020-11-2".into()),
            (5, 57, "2020-11-9".into()),
            (2, 42, "2020-12-9".into()),
            (11, 68, "2021-1-11".into()),
            (15, 32, "2021-1-17".into()),
            (12, 11, "2021-1-19".into()),
            (14, 18, "2021-1-27".into()),
        ];
        let accepted = vec![
            (10, 10, 63, 38),
            (13, 10, 73, 96),
            (7, 8, 100, 28),
            (17, 7, 119, 68),
            (20, 1, 121, 92),
            (5, 7, 42, 101),
            (2, 4, 6, 38),
            (11, 8, 37, 43),
            (15, 8, 108, 82),
            (12, 8, 38, 34),
            (14, 1, 90, 74),
        ];
        assert_eq!(
            hopper_queries_ii(drivers, rides, accepted),
            vec![
                (1, 0.0),
                (2, 0.0),
                (3, 25.0),
                (4, 0.0),
                (5, 0.0),
                (6, 20.0),
                (7, 20.0),
                (8, 20.0),
                (9, 0.0),
                (10, 0.0),
                (11, 33.33),
                (12, 16.67),
            ]
        );
    }
}
