/// LeetCode #2346 - Compute the Rank as a Percentage (SQL; Rust analogue)
use std::collections::HashMap;

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

fn rank_as_percentage(students: Vec<(i32, i32, i32)>) -> Vec<(i32, i32, f64)> {
    let mut by_dept: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    for (student_id, department_id, mark) in students {
        by_dept
            .entry(department_id)
            .or_default()
            .push((student_id, mark));
    }
    let mut ans = Vec::new();
    for (dept, group) in by_dept {
        let n = group.len() as i32;
        for &(sid, mark) in &group {
            let rank = 1 + group.iter().filter(|(_, m)| *m > mark).count() as i32;
            let pct = if n <= 1 {
                0.0
            } else {
                round2((rank - 1) as f64 * 100.0 / (n - 1) as f64)
            };
            ans.push((sid, dept, pct));
        }
    }
    ans.sort_by_key(|t| (t.1, t.0));
    ans
}

fn main() {
    println!("{:?}", rank_as_percentage(vec![]));
}

#[cfg(test)]
mod tests {
    use super::rank_as_percentage;

    #[test]
    fn example_one() {
        let students = vec![
            (2, 2, 650),
            (8, 2, 650),
            (7, 1, 920),
            (1, 1, 610),
            (3, 1, 530),
        ];
        assert_eq!(
            rank_as_percentage(students),
            vec![
                (1, 1, 50.0),
                (3, 1, 100.0),
                (7, 1, 0.0),
                (2, 2, 0.0),
                (8, 2, 0.0),
            ]
        );
    }
}
