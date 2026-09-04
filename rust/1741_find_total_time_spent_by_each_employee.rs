/// LeetCode #1741 - Find Total Time Spent by Each Employee (SQL; Rust analogue)
use std::collections::BTreeMap;

fn total_time(employees: Vec<(i32, String, i32, i32)>) -> Vec<(String, i32, i32)> {
    let mut map: BTreeMap<(String, i32), i32> = BTreeMap::new();
    for (emp, day, inn, out) in employees {
        *map.entry((day, emp)).or_insert(0) += out - inn;
    }
    map.into_iter()
        .map(|((day, emp), t)| (day, emp, t))
        .collect()
}

fn main() {
    println!("{:?}", total_time(vec![]));
}

#[cfg(test)]
mod tests {
    use super::total_time;

    #[test]
    fn example() {
        let employees = vec![
            (1, "2020-11-28".into(), 4, 32),
            (1, "2020-11-28".into(), 55, 200),
            (1, "2020-12-03".into(), 1, 42),
            (2, "2020-11-28".into(), 3, 33),
            (2, "2020-12-09".into(), 47, 74),
        ];
        assert_eq!(
            total_time(employees),
            vec![
                ("2020-11-28".into(), 1, 173),
                ("2020-11-28".into(), 2, 30),
                ("2020-12-03".into(), 1, 41),
                ("2020-12-09".into(), 2, 27),
            ]
        );
    }
}
