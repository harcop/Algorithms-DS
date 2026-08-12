/// LeetCode #3156 - Employee Task Duration and Concurrent Tasks (SQL; Rust analogue)
use std::collections::BTreeMap;

/// task: (task_id, employee_id, start_secs, end_secs)
fn employee_task_stats(tasks: Vec<(i32, i32, i64, i64)>) -> Vec<(i32, i64, i32)> {
    let mut by_emp: BTreeMap<i32, Vec<(i64, i64)>> = BTreeMap::new();
    for (_tid, eid, s, e) in tasks {
        by_emp.entry(eid).or_default().push((s, e));
    }
    let mut ans = Vec::new();
    for (eid, intervals) in by_emp {
        let mut events: Vec<(i64, i32)> = Vec::new();
        for &(s, e) in &intervals {
            events.push((s, 1));
            events.push((e, -1));
        }
        events.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        let mut cur = 0i32;
        let mut max_conc = 0i32;
        let mut prev = events[0].0;
        let mut covered = 0i64;
        for &(t, d) in &events {
            if cur > 0 {
                covered += t - prev;
            }
            cur += d;
            max_conc = max_conc.max(cur);
            prev = t;
        }
        ans.push((eid, covered / 3600, max_conc));
    }
    ans
}

fn main() {
    // 2023-05-01 as day zero; times relative within the day
    let tasks = vec![
        (1, 1001, 8 * 3600, 9 * 3600),
        (2, 1001, 8 * 3600 + 1800, 10 * 3600 + 1800),
        (3, 1001, 11 * 3600, 12 * 3600),
        (7, 1001, 13 * 3600, 15 * 3600 + 1800),
        (4, 1002, 9 * 3600, 10 * 3600),
        (5, 1002, 9 * 3600 + 1800, 11 * 3600 + 1800),
        (6, 1003, 14 * 3600, 16 * 3600),
    ];
    println!("{:?}", employee_task_stats(tasks));
}

#[cfg(test)]
mod tests {
    use super::employee_task_stats;

    #[test]
    fn example() {
        let tasks = vec![
            (1, 1001, 8 * 3600, 9 * 3600),
            (2, 1001, 8 * 3600 + 1800, 10 * 3600 + 1800),
            (3, 1001, 11 * 3600, 12 * 3600),
            (7, 1001, 13 * 3600, 15 * 3600 + 1800),
            (4, 1002, 9 * 3600, 10 * 3600),
            (5, 1002, 9 * 3600 + 1800, 11 * 3600 + 1800),
            (6, 1003, 14 * 3600, 16 * 3600),
        ];
        assert_eq!(
            employee_task_stats(tasks),
            vec![(1001, 6, 2), (1002, 2, 2), (1003, 2, 1)]
        );
    }
}
