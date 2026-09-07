/// LeetCode #3611 - Find Overbooked Employees (SQL; Rust analogue)
use std::collections::HashMap;

fn days_from_civil(y: i32, m: i32, d: i32) -> i32 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = (y - era * 400) as u32;
    let mp = if m > 2 { m - 3 } else { m + 9 } as u32;
    let doy = (153 * mp + 2) / 5 + d as u32 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe as i32 - 719468
}

fn parse_date(s: &str) -> i32 {
    let y: i32 = s[0..4].parse().unwrap();
    let m: i32 = s[5..7].parse().unwrap();
    let d: i32 = s[8..10].parse().unwrap();
    days_from_civil(y, m, d)
}

fn find_overbooked_employees(
    employees: Vec<(i32, String, String)>,
    meetings: Vec<(i32, i32, String, String, f64)>,
) -> Vec<(i32, String, String, i32)> {
    let names: HashMap<i32, (String, String)> = employees
        .into_iter()
        .map(|(id, name, dept)| (id, (name, dept)))
        .collect();
    let mut hours: HashMap<(i32, i32), f64> = HashMap::new();
    for (_mid, eid, date, _ty, dur) in meetings {
        let week = (parse_date(&date) + 3) / 7;
        *hours.entry((eid, week)).or_insert(0.0) += dur;
    }
    let mut heavy: HashMap<i32, i32> = HashMap::new();
    for ((eid, _), h) in hours {
        if h > 20.0 {
            *heavy.entry(eid).or_insert(0) += 1;
        }
    }
    let mut ans = Vec::new();
    for (eid, weeks) in heavy {
        if weeks >= 2 {
            if let Some((name, dept)) = names.get(&eid) {
                ans.push((eid, name.clone(), dept.clone(), weeks));
            }
        }
    }
    ans.sort_by(|a, b| b.3.cmp(&a.3).then(a.1.cmp(&b.1)));
    ans
}

fn main() {
    println!("{:?}", find_overbooked_employees(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::find_overbooked_employees;

    #[test]
    fn example() {
        let employees = vec![
            (1, "Alice Johnson".into(), "Engineering".into()),
            (2, "Bob Smith".into(), "Marketing".into()),
            (3, "Carol Davis".into(), "Sales".into()),
            (4, "David Wilson".into(), "Engineering".into()),
            (5, "Emma Brown".into(), "HR".into()),
        ];
        let meetings = vec![
            (1, 1, "2023-06-05".into(), "Team".into(), 8.0),
            (2, 1, "2023-06-06".into(), "Client".into(), 6.0),
            (3, 1, "2023-06-07".into(), "Training".into(), 7.0),
            (4, 1, "2023-06-12".into(), "Team".into(), 12.0),
            (5, 1, "2023-06-13".into(), "Client".into(), 9.0),
            (6, 2, "2023-06-05".into(), "Team".into(), 15.0),
            (7, 2, "2023-06-06".into(), "Client".into(), 8.0),
            (8, 2, "2023-06-12".into(), "Training".into(), 10.0),
            (9, 3, "2023-06-05".into(), "Team".into(), 4.0),
            (10, 3, "2023-06-06".into(), "Client".into(), 3.0),
            (11, 4, "2023-06-05".into(), "Team".into(), 25.0),
            (12, 4, "2023-06-19".into(), "Client".into(), 22.0),
            (13, 5, "2023-06-05".into(), "Training".into(), 2.0),
        ];
        let ans = find_overbooked_employees(employees, meetings);
        assert_eq!(ans.len(), 2);
        assert_eq!(ans[0].0, 1);
        assert_eq!(ans[0].1, "Alice Johnson");
        assert_eq!(ans[0].3, 2);
        assert_eq!(ans[1].0, 4);
        assert_eq!(ans[1].1, "David Wilson");
        assert_eq!(ans[1].3, 2);
    }
}
