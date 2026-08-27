/// LeetCode #3450 - Maximum Students on a Single Bench
fn max_students_on_bench(students: Vec<Vec<i32>>) -> i32 {
    use std::collections::{HashMap, HashSet};
    if students.is_empty() {
        return 0;
    }
    let mut d: HashMap<i32, HashSet<i32>> = HashMap::new();
    for s in students {
        d.entry(s[1]).or_default().insert(s[0]);
    }
    d.values().map(|v| v.len()).max().unwrap_or(0) as i32
}

fn main() {
    println!(
        "{}",
        max_students_on_bench(vec![
            vec![1, 2],
            vec![2, 2],
            vec![3, 3],
            vec![1, 3],
            vec![2, 3]
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::max_students_on_bench;

    #[test]
    fn example1() {
        assert_eq!(
            max_students_on_bench(vec![
                vec![1, 2],
                vec![2, 2],
                vec![3, 3],
                vec![1, 3],
                vec![2, 3]
            ]),
            3
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_students_on_bench(vec![
                vec![1, 1],
                vec![2, 1],
                vec![3, 1],
                vec![4, 2],
                vec![5, 2]
            ]),
            3
        );
    }

    #[test]
    fn example3() {
        assert_eq!(max_students_on_bench(vec![vec![1, 1], vec![1, 1]]), 1);
    }

    #[test]
    fn example4() {
        assert_eq!(max_students_on_bench(vec![]), 0);
    }
}
