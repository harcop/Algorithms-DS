/// LeetCode #3601 - Find Drivers with Improved Fuel Efficiency (SQL; Rust analogue)
use std::collections::HashMap;

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

fn month(date: &str) -> i32 {
    date[5..7].parse().unwrap()
}

fn find_improved_efficiency_drivers(
    drivers: Vec<(i32, String)>,
    trips: Vec<(i32, i32, String, f64, f64)>,
) -> Vec<(i32, String, f64, f64, f64)> {
    let names: HashMap<i32, String> = drivers.into_iter().collect();
    let mut half: HashMap<i32, (f64, i32, f64, i32)> = HashMap::new();
    for (_tid, did, date, dist, fuel) in trips {
        let eff = dist / fuel;
        let e = half.entry(did).or_insert((0.0, 0, 0.0, 0));
        if month(&date) <= 6 {
            e.0 += eff;
            e.1 += 1;
        } else {
            e.2 += eff;
            e.3 += 1;
        }
    }
    let mut ans = Vec::new();
    for (did, (s1, c1, s2, c2)) in half {
        if c1 == 0 || c2 == 0 {
            continue;
        }
        let a1 = s1 / c1 as f64;
        let a2 = s2 / c2 as f64;
        if a2 > a1 {
            if let Some(name) = names.get(&did) {
                ans.push((did, name.clone(), round2(a1), round2(a2), round2(a2 - a1)));
            }
        }
    }
    ans.sort_by(|a, b| {
        b.4.partial_cmp(&a.4)
            .unwrap()
            .then(a.1.cmp(&b.1))
    });
    ans
}

fn main() {
    println!("{:?}", find_improved_efficiency_drivers(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::find_improved_efficiency_drivers;

    fn close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-9, "{} vs {}", a, b);
    }

    #[test]
    fn example() {
        let drivers = vec![
            (1, "Alice Johnson".into()),
            (2, "Bob Smith".into()),
            (3, "Carol Davis".into()),
            (4, "David Wilson".into()),
            (5, "Emma Brown".into()),
        ];
        let trips = vec![
            (1, 1, "2023-02-15".into(), 120.5, 10.2),
            (2, 1, "2023-03-20".into(), 200.0, 16.5),
            (3, 1, "2023-08-10".into(), 150.0, 11.0),
            (4, 1, "2023-09-25".into(), 180.0, 12.5),
            (5, 2, "2023-01-10".into(), 100.0, 9.0),
            (6, 2, "2023-04-15".into(), 250.0, 22.0),
            (7, 2, "2023-10-05".into(), 200.0, 15.0),
            (8, 3, "2023-03-12".into(), 80.0, 8.5),
            (9, 3, "2023-05-18".into(), 90.0, 9.2),
            (10, 4, "2023-07-22".into(), 160.0, 12.8),
            (11, 4, "2023-11-30".into(), 140.0, 11.0),
            (12, 5, "2023-02-28".into(), 110.0, 11.5),
        ];
        let ans = find_improved_efficiency_drivers(drivers, trips);
        assert_eq!(ans.len(), 2);
        assert_eq!(ans[0].0, 2);
        assert_eq!(ans[0].1, "Bob Smith");
        close(ans[0].2, 11.24);
        close(ans[0].3, 13.33);
        close(ans[0].4, 2.10);
        assert_eq!(ans[1].0, 1);
        assert_eq!(ans[1].1, "Alice Johnson");
        close(ans[1].2, 11.97);
        close(ans[1].3, 14.02);
        close(ans[1].4, 2.05);
    }
}
