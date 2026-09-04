/// LeetCode #1270 - All People Report to the Given Manager (SQL; Rust analogue)
use std::collections::HashMap;

fn people_report_to_manager(employees: Vec<(i32, String, i32)>) -> Vec<i32> {
    let mgr: HashMap<i32, i32> = employees.iter().map(|(id, _, m)| (*id, *m)).collect();
    let mut ans = Vec::new();
    for (id, _, _) in &employees {
        if *id == 1 {
            continue;
        }
        let mut cur = *id;
        let mut ok = false;
        for _ in 0..3 {
            if let Some(&m) = mgr.get(&cur) {
                if m == 1 {
                    ok = true;
                    break;
                }
                if m == cur {
                    break;
                }
                cur = m;
            } else {
                break;
            }
        }
        if ok {
            ans.push(*id);
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
    use super::people_report_to_manager;

    #[test]
    fn example() {
        let employees = vec![
            (1, "Boss".into(), 1),
            (3, "Alice".into(), 3),
            (2, "Bob".into(), 1),
            (4, "Daniel".into(), 2),
            (7, "Luis".into(), 4),
            (8, "Jhon".into(), 3),
            (9, "Angela".into(), 8),
            (77, "Robert".into(), 1),
        ];
        assert_eq!(people_report_to_manager(employees), vec![2, 4, 7, 77]);
    }
}
