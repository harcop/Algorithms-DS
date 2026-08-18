/// LeetCode #3268 - Find Overlapping Shifts II (SQL; Rust analogue)
/// employee_shifts: (employee_id, start_time, end_time) as "YYYY-MM-DD HH:MM:SS"
fn find_overlapping_shifts_ii(
    employee_shifts: Vec<(i32, String, String)>,
) -> Vec<(i32, i32, i64)> {
    use std::collections::BTreeMap;
    let parse = |s: &str| -> i64 {
        let y: i64 = s[0..4].parse().unwrap();
        let mo: i64 = s[5..7].parse().unwrap();
        let d: i64 = s[8..10].parse().unwrap();
        let h: i64 = s[11..13].parse().unwrap();
        let mi: i64 = s[14..16].parse().unwrap();
        let mdays = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut days = y * 365;
        for m in 1..mo as usize {
            days += mdays[m];
        }
        days += d;
        days * 24 * 60 + h * 60 + mi
    };
    let mut by_emp: BTreeMap<i32, Vec<(i64, i64)>> = BTreeMap::new();
    for (eid, st, ed) in employee_shifts {
        by_emp.entry(eid).or_default().push((parse(&st), parse(&ed)));
    }
    let mut ans = Vec::new();
    for (eid, shifts) in by_emp {
        let mut events: Vec<(i64, i32)> = Vec::new();
        for &(s, e) in &shifts {
            events.push((s, 1));
            events.push((e, -1));
        }
        events.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        let mut cur = 0i32;
        let mut max_conc = 0i32;
        for &(_, d) in &events {
            cur += d;
            max_conc = max_conc.max(cur);
        }
        let mut total = 0i64;
        for i in 0..shifts.len() {
            for j in i + 1..shifts.len() {
                let a = shifts[i].0.max(shifts[j].0);
                let b = shifts[i].1.min(shifts[j].1);
                if b > a {
                    total += b - a;
                }
            }
        }
        ans.push((eid, max_conc, total));
    }
    ans
}

fn main() {
    let shifts = vec![(
        1,
        "2023-10-01 09:00:00".into(),
        "2023-10-01 17:00:00".into(),
    )];
    println!("{:?}", find_overlapping_shifts_ii(shifts));
}

#[cfg(test)]
mod tests {
    use super::find_overlapping_shifts_ii;

    #[test]
    fn example() {
        let shifts = vec![
            (
                1,
                "2023-10-01 09:00:00".into(),
                "2023-10-01 17:00:00".into(),
            ),
            (
                1,
                "2023-10-01 15:00:00".into(),
                "2023-10-01 23:00:00".into(),
            ),
            (
                1,
                "2023-10-01 16:00:00".into(),
                "2023-10-02 00:00:00".into(),
            ),
            (
                2,
                "2023-10-01 09:00:00".into(),
                "2023-10-01 17:00:00".into(),
            ),
            (
                2,
                "2023-10-01 11:00:00".into(),
                "2023-10-01 19:00:00".into(),
            ),
            (
                3,
                "2023-10-01 09:00:00".into(),
                "2023-10-01 17:00:00".into(),
            ),
        ];
        assert_eq!(
            find_overlapping_shifts_ii(shifts),
            vec![(1, 3, 600), (2, 2, 360), (3, 1, 0)]
        );
    }
}
