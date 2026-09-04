/// LeetCode #1112 - Highest Grade For Each Student (SQL; Rust analogue)
use std::collections::HashMap;

fn highest_grade(enrollments: Vec<(i32, i32, i32)>) -> Vec<(i32, i32, i32)> {
    let mut best: HashMap<i32, (i32, i32)> = HashMap::new();
    for (sid, cid, grade) in enrollments {
        best.entry(sid)
            .and_modify(|e| {
                if grade > e.1 || (grade == e.1 && cid < e.0) {
                    *e = (cid, grade);
                }
            })
            .or_insert((cid, grade));
    }
    let mut ans: Vec<(i32, i32, i32)> = best.into_iter().map(|(s, (c, g))| (s, c, g)).collect();
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::highest_grade;

    #[test]
    fn example() {
        let enrollments = vec![
            (2, 2, 95),
            (2, 3, 95),
            (1, 1, 90),
            (1, 2, 99),
            (3, 1, 80),
            (3, 2, 75),
            (3, 3, 82),
        ];
        assert_eq!(
            highest_grade(enrollments),
            vec![(1, 2, 99), (2, 2, 95), (3, 3, 82)]
        );
    }
}
