/// LeetCode #579 - Find Cumulative Salary of an Employee (SQL; Rust analogue)
use std::collections::HashMap;

fn cumulative_salary(employee: Vec<(i32, i32, i32)>) -> Vec<(i32, i32, i32)> {
    let mut by_id: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
    for (id, month, salary) in employee {
        by_id.entry(id).or_default().insert(month, salary);
    }
    let mut ans = Vec::new();
    for (id, months) in by_id {
        let max_m = *months.keys().max().unwrap();
        for (&m, _) in &months {
            if m == max_m {
                continue;
            }
            let sum = (m - 2..=m).filter_map(|k| months.get(&k)).sum();
            ans.push((id, m, sum));
        }
    }
    ans.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::cumulative_salary;

    #[test]
    fn example() {
        let employee = vec![
            (1, 1, 20),
            (2, 1, 20),
            (1, 2, 30),
            (2, 2, 30),
            (3, 2, 40),
            (1, 3, 40),
            (3, 3, 60),
            (1, 4, 60),
            (3, 4, 70),
            (1, 7, 90),
            (1, 8, 90),
        ];
        assert_eq!(
            cumulative_salary(employee),
            vec![
                (1, 7, 90),
                (1, 4, 130),
                (1, 3, 90),
                (1, 2, 50),
                (1, 1, 20),
                (2, 1, 20),
                (3, 3, 100),
                (3, 2, 40),
            ]
        );
    }
}
