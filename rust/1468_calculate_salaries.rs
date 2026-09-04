/// LeetCode #1468 - Calculate Salaries (SQL; Rust analogue)
use std::collections::HashMap;

fn calculate_salaries(
    salaries: Vec<(i32, i32, String, i32)>,
) -> Vec<(i32, i32, String, i32)> {
    let mut max_sal: HashMap<i32, i32> = HashMap::new();
    for &(cid, _, _, sal) in &salaries {
        max_sal
            .entry(cid)
            .and_modify(|m| *m = (*m).max(sal))
            .or_insert(sal);
    }
    salaries
        .into_iter()
        .map(|(cid, eid, name, sal)| {
            let top = max_sal[&cid];
            let rate = if top < 1000 {
                1.0
            } else if top <= 10000 {
                0.76
            } else {
                0.51
            };
            let after = (sal as f64 * rate).round() as i32;
            (cid, eid, name, after)
        })
        .collect()
}

fn main() {
    println!("{:?}", calculate_salaries(vec![]));
}

#[cfg(test)]
mod tests {
    use super::calculate_salaries;

    #[test]
    fn example() {
        let salaries = vec![
            (1, 1, "Tony".into(), 2000),
            (1, 2, "Pronub".into(), 21300),
            (1, 3, "Tyrrox".into(), 10800),
            (2, 1, "Pam".into(), 300),
            (2, 7, "Bassem".into(), 450),
            (2, 9, "Hermione".into(), 700),
            (3, 7, "Bocaben".into(), 100),
            (3, 2, "Ognjen".into(), 2200),
            (3, 13, "Nyancat".into(), 3300),
            (3, 15, "Morninngcat".into(), 7777),
        ];
        let mut got = calculate_salaries(salaries);
        got.sort_by_key(|r| (r.0, r.1));
        assert_eq!(
            got,
            vec![
                (1, 1, "Tony".into(), 1020),
                (1, 2, "Pronub".into(), 10863),
                (1, 3, "Tyrrox".into(), 5508),
                (2, 1, "Pam".into(), 300),
                (2, 7, "Bassem".into(), 450),
                (2, 9, "Hermione".into(), 700),
                (3, 2, "Ognjen".into(), 1672),
                (3, 7, "Bocaben".into(), 76),
                (3, 13, "Nyancat".into(), 2508),
                (3, 15, "Morninngcat".into(), 5911),
            ]
        );
    }
}
