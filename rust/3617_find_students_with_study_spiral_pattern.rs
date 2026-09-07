/// LeetCode #3617 - Find Students with Study Spiral Pattern (SQL; Rust analogue)
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

fn check_pattern(subjects: &[String], hours: f64) -> Option<(i32, f64)> {
    let n = subjects.len();
    if n < 6 {
        return None;
    }
    for cycle_len in 3..=n / 2 {
        if n % cycle_len != 0 {
            continue;
        }
        let first = &subjects[..cycle_len];
        let mut ok = true;
        for i in 1..n / cycle_len {
            if &subjects[i * cycle_len..(i + 1) * cycle_len] != first {
                ok = false;
                break;
            }
        }
        if ok {
            return Some((cycle_len as i32, hours));
        }
    }
    None
}

fn find_study_spiral_pattern(
    students: Vec<(i32, String, String)>,
    study_sessions: Vec<(i32, i32, String, String, f64)>,
) -> Vec<(i32, String, String, i32, f64)> {
    let info: HashMap<i32, (String, String)> = students
        .into_iter()
        .map(|(id, name, major)| (id, (name, major)))
        .collect();
    let mut by_student: HashMap<i32, Vec<(i32, String, f64)>> = HashMap::new();
    for (_sid, st, subject, date, hours) in study_sessions {
        by_student
            .entry(st)
            .or_default()
            .push((parse_date(&date), subject, hours));
    }
    let mut best: HashMap<i32, (i32, f64)> = HashMap::new();
    for (st, mut sess) in by_student {
        sess.sort_by_key(|x| x.0);
        let mut temp: Vec<(String, f64)> = Vec::new();
        let mut last_date = 0;
        let mut flush = |temp: &mut Vec<(String, f64)>| {
            if temp.len() >= 6 {
                let subjects: Vec<String> = temp.iter().map(|x| x.0.clone()).collect();
                let hours: f64 = temp.iter().map(|x| x.1).sum();
                if let Some((cl, th)) = check_pattern(&subjects, hours) {
                    best.entry(st)
                        .and_modify(|e| {
                            if cl > e.0 || (cl == e.0 && th > e.1) {
                                *e = (cl, th);
                            }
                        })
                        .or_insert((cl, th));
                }
            }
            temp.clear();
        };
        for (day, subject, hours) in sess {
            if temp.is_empty() {
                temp.push((subject, hours));
            } else if day - last_date <= 2 {
                temp.push((subject, hours));
            } else {
                flush(&mut temp);
                temp.push((subject, hours));
            }
            last_date = day;
        }
        flush(&mut temp);
    }
    let mut ans = Vec::new();
    for (id, (cl, hours)) in best {
        if let Some((name, major)) = info.get(&id) {
            ans.push((id, name.clone(), major.clone(), cl, hours));
        }
    }
    ans.sort_by(|a, b| {
        b.3.cmp(&a.3)
            .then(b.4.partial_cmp(&a.4).unwrap())
    });
    ans
}

fn main() {
    println!("{:?}", find_study_spiral_pattern(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::find_study_spiral_pattern;

    fn close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-9, "{} vs {}", a, b);
    }

    #[test]
    fn example() {
        let students = vec![
            (1, "Alice Chen".into(), "Computer Science".into()),
            (2, "Bob Johnson".into(), "Mathematics".into()),
            (3, "Carol Davis".into(), "Physics".into()),
            (4, "David Wilson".into(), "Chemistry".into()),
            (5, "Emma Brown".into(), "Biology".into()),
        ];
        let sessions = vec![
            (1, 1, "Math".into(), "2023-10-01".into(), 2.5),
            (2, 1, "Physics".into(), "2023-10-02".into(), 3.0),
            (3, 1, "Chemistry".into(), "2023-10-03".into(), 2.0),
            (4, 1, "Math".into(), "2023-10-04".into(), 2.5),
            (5, 1, "Physics".into(), "2023-10-05".into(), 3.0),
            (6, 1, "Chemistry".into(), "2023-10-06".into(), 2.0),
            (7, 2, "Algebra".into(), "2023-10-01".into(), 4.0),
            (8, 2, "Calculus".into(), "2023-10-02".into(), 3.5),
            (9, 2, "Statistics".into(), "2023-10-03".into(), 2.5),
            (10, 2, "Geometry".into(), "2023-10-04".into(), 3.0),
            (11, 2, "Algebra".into(), "2023-10-05".into(), 4.0),
            (12, 2, "Calculus".into(), "2023-10-06".into(), 3.5),
            (13, 2, "Statistics".into(), "2023-10-07".into(), 2.5),
            (14, 2, "Geometry".into(), "2023-10-08".into(), 3.0),
            (15, 3, "Biology".into(), "2023-10-01".into(), 2.0),
            (16, 3, "Chemistry".into(), "2023-10-02".into(), 2.5),
            (17, 3, "Biology".into(), "2023-10-03".into(), 2.0),
            (18, 3, "Chemistry".into(), "2023-10-04".into(), 2.5),
            (19, 4, "Organic".into(), "2023-10-01".into(), 3.0),
            (20, 4, "Physical".into(), "2023-10-05".into(), 2.5),
        ];
        let ans = find_study_spiral_pattern(students, sessions);
        assert_eq!(ans.len(), 2);
        assert_eq!(ans[0].0, 2);
        assert_eq!(ans[0].1, "Bob Johnson");
        assert_eq!(ans[0].3, 4);
        close(ans[0].4, 26.0);
        assert_eq!(ans[1].0, 1);
        assert_eq!(ans[1].1, "Alice Chen");
        assert_eq!(ans[1].3, 3);
        close(ans[1].4, 15.0);
    }
}
