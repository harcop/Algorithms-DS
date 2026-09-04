/// LeetCode #1651 - Hopper Company Queries III (SQL; Rust analogue)
use std::collections::HashMap;

fn parse_ymd(s: &str) -> (i32, u32, u32) {
    let mut p = s.split('-');
    (
        p.next().unwrap().parse().unwrap(),
        p.next().unwrap().parse().unwrap(),
        p.next().unwrap().parse().unwrap(),
    )
}

fn hopper_queries_iii(
    _drivers: Vec<(i32, String)>,
    rides: Vec<(i32, i32, String)>,
    accepted: Vec<(i32, i32, i32, i32)>,
) -> Vec<(i32, f64, f64)> {
    let acc: HashMap<i32, (i32, i32)> = accepted
        .into_iter()
        .map(|(id, _, dist, dur)| (id, (dist, dur)))
        .collect();
    let mut monthly = [(0i32, 0i32); 13];
    for (id, _, req) in rides {
        let (y, m, _) = parse_ymd(&req);
        if y == 2020 {
            if let Some(&(dist, dur)) = acc.get(&id) {
                monthly[m as usize].0 += dist;
                monthly[m as usize].1 += dur;
            }
        }
    }
    let mut ans = Vec::new();
    for month in 1..=10 {
        let dist = (monthly[month].0 + monthly[month + 1].0 + monthly[month + 2].0) as f64 / 3.0;
        let dur = (monthly[month].1 + monthly[month + 1].1 + monthly[month + 2].1) as f64 / 3.0;
        ans.push((
            month as i32,
            (dist * 100.0).round() / 100.0,
            (dur * 100.0).round() / 100.0,
        ));
    }
    ans
}

fn main() {
    println!("{:?}", hopper_queries_iii(vec![], vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::hopper_queries_iii;

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
            hopper_queries_iii(drivers, rides, accepted),
            vec![
                (1, 21.0, 12.67),
                (2, 21.0, 12.67),
                (3, 21.0, 12.67),
                (4, 24.33, 32.0),
                (5, 57.67, 41.33),
                (6, 97.33, 64.0),
                (7, 73.0, 32.0),
                (8, 39.67, 22.67),
                (9, 54.33, 64.33),
                (10, 56.33, 77.0),
            ]
        );
    }
}
