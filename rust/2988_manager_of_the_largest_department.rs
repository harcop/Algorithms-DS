/// LeetCode #2988 - Manager of the Largest Department (SQL; Rust analogue)
use std::collections::HashMap;

fn managers_of_largest_department(employees: Vec<(i32, String, i32, String)>) -> Vec<(String, i32)> {
    // (emp_id, emp_name, dep_id, position)
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for (_, _, dep_id, _) in &employees {
        *cnt.entry(*dep_id).or_default() += 1;
    }
    let max_cnt = *cnt.values().max().unwrap_or(&0);
    let mut ans: Vec<_> = employees
        .into_iter()
        .filter(|(_, _, dep_id, pos)| pos == "Manager" && cnt[dep_id] == max_cnt)
        .map(|(_, name, dep_id, _)| (name, dep_id))
        .collect();
    ans.sort_by_key(|(_, dep)| *dep);
    ans
}

fn main() {
    let employees = vec![
        (156, "Michael".into(), 107, "Manager".into()),
        (112, "Lucas".into(), 107, "Consultant".into()),
        (8, "Isabella".into(), 101, "Manager".into()),
        (160, "Joseph".into(), 100, "Manager".into()),
        (80, "Aiden".into(), 100, "Engineer".into()),
        (190, "Skylar".into(), 100, "Freelancer".into()),
        (196, "Stella".into(), 101, "Coordinator".into()),
        (167, "Audrey".into(), 100, "Consultant".into()),
        (97, "Nathan".into(), 101, "Supervisor".into()),
        (128, "Ian".into(), 101, "Administrator".into()),
        (81, "Ethan".into(), 107, "Administrator".into()),
    ];
    println!("{:?}", managers_of_largest_department(employees));
}

#[cfg(test)]
mod tests {
    use super::managers_of_largest_department;

    #[test]
    fn example() {
        let employees = vec![
            (156, "Michael".into(), 107, "Manager".into()),
            (112, "Lucas".into(), 107, "Consultant".into()),
            (8, "Isabella".into(), 101, "Manager".into()),
            (160, "Joseph".into(), 100, "Manager".into()),
            (80, "Aiden".into(), 100, "Engineer".into()),
            (190, "Skylar".into(), 100, "Freelancer".into()),
            (196, "Stella".into(), 101, "Coordinator".into()),
            (167, "Audrey".into(), 100, "Consultant".into()),
            (97, "Nathan".into(), 101, "Supervisor".into()),
            (128, "Ian".into(), 101, "Administrator".into()),
            (81, "Ethan".into(), 107, "Administrator".into()),
        ];
        assert_eq!(
            managers_of_largest_department(employees),
            vec![("Joseph".into(), 100), ("Isabella".into(), 101)]
        );
    }
}
