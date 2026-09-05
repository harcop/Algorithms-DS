/// LeetCode #3586 - Find COVID Recovery Patients (SQL; Rust analogue)
use std::collections::HashMap;

fn days(s: &str) -> i32 {
    let y: i32 = s[0..4].parse().unwrap();
    let m: i32 = s[5..7].parse().unwrap();
    let d: i32 = s[8..10].parse().unwrap();
    let a = (14 - m) / 12;
    let y = y + 4800 - a;
    let m = m + 12 * a - 3;
    d + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045
}

fn find_covid_recovery_patients(
    patients: Vec<(i32, String, i32)>,
    tests: Vec<(i32, i32, String, String)>,
) -> Vec<(i32, String, i32, i32)> {
    let mut first_pos: HashMap<i32, String> = HashMap::new();
    for (_tid, pid, date, result) in &tests {
        if result == "Positive" {
            first_pos
                .entry(*pid)
                .and_modify(|d| {
                    if date < d {
                        *d = date.clone();
                    }
                })
                .or_insert(date.clone());
        }
    }
    let mut first_neg: HashMap<i32, String> = HashMap::new();
    for (_tid, pid, date, result) in &tests {
        if result == "Negative" {
            if let Some(pos) = first_pos.get(pid) {
                if date > pos {
                    first_neg
                        .entry(*pid)
                        .and_modify(|d| {
                            if date < d {
                                *d = date.clone();
                            }
                        })
                        .or_insert(date.clone());
                }
            }
        }
    }
    let mut ans = Vec::new();
    for (pid, name, age) in patients {
        if let (Some(pos), Some(neg)) = (first_pos.get(&pid), first_neg.get(&pid)) {
            ans.push((pid, name, age, days(neg) - days(pos)));
        }
    }
    ans.sort_by(|a, b| a.3.cmp(&b.3).then(a.1.cmp(&b.1)));
    ans
}

fn main() {
    println!("{:?}", find_covid_recovery_patients(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::find_covid_recovery_patients;

    #[test]
    fn example() {
        let patients = vec![
            (1, "Alice Smith".into(), 28),
            (2, "Bob Johnson".into(), 35),
            (3, "Carol Davis".into(), 42),
            (4, "David Wilson".into(), 31),
            (5, "Emma Brown".into(), 29),
        ];
        let tests = vec![
            (1, 1, "2023-01-15".into(), "Positive".into()),
            (2, 1, "2023-01-25".into(), "Negative".into()),
            (3, 2, "2023-02-01".into(), "Positive".into()),
            (4, 2, "2023-02-05".into(), "Inconclusive".into()),
            (5, 2, "2023-02-12".into(), "Negative".into()),
            (6, 3, "2023-01-20".into(), "Negative".into()),
            (7, 3, "2023-02-10".into(), "Positive".into()),
            (8, 3, "2023-02-20".into(), "Negative".into()),
            (9, 4, "2023-01-10".into(), "Positive".into()),
            (10, 4, "2023-01-18".into(), "Positive".into()),
            (11, 5, "2023-02-15".into(), "Negative".into()),
            (12, 5, "2023-02-20".into(), "Negative".into()),
        ];
        assert_eq!(
            find_covid_recovery_patients(patients, tests),
            vec![
                (1, "Alice Smith".into(), 28, 10),
                (3, "Carol Davis".into(), 42, 10),
                (2, "Bob Johnson".into(), 35, 11),
            ]
        );
    }
}
