/// LeetCode #3262 - Find Overlapping Shifts (SQL; Rust analogue)
/// employee_shifts: (employee_id, start_time, end_time) as "HH:MM:SS"
fn find_overlapping_shifts(employee_shifts: Vec<(i32, String, String)>) -> Vec<(i32, i32)> {
    use std::collections::BTreeMap;
    let parse = |s: &str| -> i32 {
        let p: Vec<i32> = s.split(':').map(|x| x.parse().unwrap()).collect();
        p[0] * 3600 + p[1] * 60 + p[2]
    };
    let mut by_emp: BTreeMap<i32, Vec<(i32, i32)>> = BTreeMap::new();
    for (eid, st, ed) in employee_shifts {
        by_emp.entry(eid).or_default().push((parse(&st), parse(&ed)));
    }
    let mut ans = Vec::new();
    for (eid, shifts) in by_emp {
        let mut cnt = 0;
        for i in 0..shifts.len() {
            for j in 0..shifts.len() {
                if i == j {
                    continue;
                }
                if shifts[i].0 < shifts[j].0 && shifts[i].1 > shifts[j].0 {
                    cnt += 1;
                }
            }
        }
        if cnt > 0 {
            ans.push((eid, cnt));
        }
    }
    ans
}

fn main() {
    let shifts = vec![
        (1, "08:00:00".into(), "12:00:00".into()),
        (1, "11:00:00".into(), "15:00:00".into()),
    ];
    println!("{:?}", find_overlapping_shifts(shifts));
}

#[cfg(test)]
mod tests {
    use super::find_overlapping_shifts;

    #[test]
    fn example() {
        let shifts = vec![
            (1, "08:00:00".into(), "12:00:00".into()),
            (1, "11:00:00".into(), "15:00:00".into()),
            (1, "14:00:00".into(), "18:00:00".into()),
            (2, "09:00:00".into(), "17:00:00".into()),
            (2, "16:00:00".into(), "20:00:00".into()),
            (3, "10:00:00".into(), "12:00:00".into()),
            (3, "13:00:00".into(), "15:00:00".into()),
            (3, "16:00:00".into(), "18:00:00".into()),
            (4, "08:00:00".into(), "10:00:00".into()),
            (4, "09:00:00".into(), "11:00:00".into()),
        ];
        assert_eq!(
            find_overlapping_shifts(shifts),
            vec![(1, 2), (2, 1), (4, 1)]
        );
    }
}
