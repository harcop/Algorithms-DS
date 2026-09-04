/// LeetCode #1661 - Average Time of Process per Machine (SQL; Rust analogue)
use std::collections::HashMap;

fn average_process_time(
    activity: Vec<(i32, i32, String, f64)>,
) -> Vec<(i32, f64)> {
    let mut start: HashMap<(i32, i32), f64> = HashMap::new();
    let mut sums: HashMap<i32, (f64, i32)> = HashMap::new();
    for (machine, process, ty, ts) in activity {
        if ty == "start" {
            start.insert((machine, process), ts);
        } else {
            let s = start[&(machine, process)];
            let e = sums.entry(machine).or_insert((0.0, 0));
            e.0 += ts - s;
            e.1 += 1;
        }
    }
    let mut ans: Vec<(i32, f64)> = sums
        .into_iter()
        .map(|(m, (sum, n))| (m, (sum / n as f64 * 1000.0).round() / 1000.0))
        .collect();
    ans.sort_by_key(|r| r.0);
    ans
}

fn main() {
    println!("{:?}", average_process_time(vec![]));
}

#[cfg(test)]
mod tests {
    use super::average_process_time;

    #[test]
    fn example() {
        let activity = vec![
            (0, 0, "start".into(), 0.712),
            (0, 0, "end".into(), 1.520),
            (0, 1, "start".into(), 3.140),
            (0, 1, "end".into(), 4.120),
            (1, 0, "start".into(), 0.550),
            (1, 0, "end".into(), 1.550),
            (1, 1, "start".into(), 0.430),
            (1, 1, "end".into(), 1.420),
            (2, 0, "start".into(), 4.100),
            (2, 0, "end".into(), 4.512),
            (2, 1, "start".into(), 2.500),
            (2, 1, "end".into(), 5.000),
        ];
        assert_eq!(
            average_process_time(activity),
            vec![(0, 0.894), (1, 0.995), (2, 1.456)]
        );
    }
}
