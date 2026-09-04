/// LeetCode #615 - Average Salary: Departments VS Company (SQL; Rust analogue)

fn year_month(s: &str) -> String {
    let p: Vec<&str> = s
        .split(|c: char| !c.is_ascii_digit())
        .filter(|x| !x.is_empty())
        .collect();
    format!("{}-{:02}", p[0], p[1].parse::<i32>().unwrap())
}

use std::collections::HashMap;

fn department_vs_company(
    salary: Vec<(i32, i32, i32, String)>,
    employee: Vec<(i32, i32)>,
) -> Vec<(String, i32, String)> {
    let dept: HashMap<i32, i32> = employee.into_iter().collect();
    let mut company: HashMap<String, (i64, i32)> = HashMap::new();
    let mut department: HashMap<(String, i32), (i64, i32)> = HashMap::new();
    for (_, eid, amount, pay_date) in salary {
        let month = year_month(&pay_date);
        let d = dept[&eid];
        let c = company.entry(month.clone()).or_insert((0, 0));
        c.0 += amount as i64;
        c.1 += 1;
        let e = department.entry((month, d)).or_insert((0, 0));
        e.0 += amount as i64;
        e.1 += 1;
    }
    let mut ans = Vec::new();
    for ((month, d), (sum, n)) in department {
        let (cs, cn) = company[&month];
        let da = sum as f64 / n as f64;
        let ca = cs as f64 / cn as f64;
        let cmp = if (da - ca).abs() < 1e-9 {
            "same"
        } else if da > ca {
            "higher"
        } else {
            "lower"
        };
        ans.push((month, d, cmp.to_string()));
    }
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::department_vs_company;

    #[test]
    fn example() {
        let salary = vec![
            (1, 1, 9000, "2017-03-31".into()),
            (2, 2, 6000, "2017-03-31".into()),
            (3, 3, 10000, "2017-03-31".into()),
            (4, 1, 7000, "2017-02-28".into()),
            (5, 2, 6000, "2017-02-28".into()),
            (6, 3, 8000, "2017-02-28".into()),
        ];
        let employee = vec![(1, 1), (2, 2), (3, 2)];
        assert_eq!(
            department_vs_company(salary, employee),
            vec![
                ("2017-02".into(), 1, "same".into()),
                ("2017-02".into(), 2, "same".into()),
                ("2017-03".into(), 1, "higher".into()),
                ("2017-03".into(), 2, "lower".into()),
            ]
        );
    }
}
