/// LeetCode #577 - Employee Bonus (SQL; Rust analogue)
use std::collections::HashMap;

fn employee_bonus(
    employee: Vec<(i32, String, Option<i32>, i32)>,
    bonus: Vec<(i32, i32)>,
) -> Vec<(String, Option<i32>)> {
    let bonus: HashMap<i32, i32> = bonus.into_iter().collect();
    let mut ans = Vec::new();
    for (emp_id, name, _, _) in employee {
        let b = bonus.get(&emp_id).copied();
        if b.unwrap_or(0) < 1000 {
            ans.push((name, b));
        }
    }
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::employee_bonus;

    #[test]
    fn example() {
        let employee = vec![
            (3, "Brad".into(), None, 4000),
            (1, "John".into(), Some(3), 1000),
            (2, "Dan".into(), Some(3), 2000),
            (4, "Thomas".into(), Some(3), 4000),
        ];
        let bonus = vec![(2, 500), (4, 2000)];
        assert_eq!(
            employee_bonus(employee, bonus),
            vec![
                ("Brad".into(), None),
                ("Dan".into(), Some(500)),
                ("John".into(), None),
            ]
        );
    }
}
