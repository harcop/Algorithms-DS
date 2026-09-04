/// LeetCode #1699 - Number of Calls Between Two Persons (SQL; Rust analogue)
use std::collections::BTreeMap;

fn calls_between_persons(calls: Vec<(i32, i32, i32)>) -> Vec<(i32, i32, i32, i32)> {
    let mut map: BTreeMap<(i32, i32), (i32, i32)> = BTreeMap::new();
    for (a, b, dur) in calls {
        let key = if a < b { (a, b) } else { (b, a) };
        let e = map.entry(key).or_insert((0, 0));
        e.0 += 1;
        e.1 += dur;
    }
    map.into_iter()
        .map(|((p1, p2), (cnt, tot))| (p1, p2, cnt, tot))
        .collect()
}

fn main() {
    println!("{:?}", calls_between_persons(vec![]));
}

#[cfg(test)]
mod tests {
    use super::calls_between_persons;

    #[test]
    fn example() {
        let calls = vec![
            (1, 2, 59),
            (2, 1, 11),
            (1, 3, 20),
            (3, 4, 100),
            (3, 4, 200),
            (3, 4, 200),
            (4, 3, 499),
        ];
        assert_eq!(
            calls_between_persons(calls),
            vec![(1, 2, 2, 70), (1, 3, 1, 20), (3, 4, 4, 999)]
        );
    }
}
