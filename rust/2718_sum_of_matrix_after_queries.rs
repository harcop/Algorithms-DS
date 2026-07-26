/// LeetCode #2718 - Sum of Matrix After Queries
use std::collections::HashSet;

fn matrix_sum_queries(n: i32, queries: Vec<Vec<i32>>) -> i64 {
    let n = n as i64;
    let mut row = HashSet::new();
    let mut col = HashSet::new();
    let mut ans = 0i64;
    for q in queries.iter().rev() {
        let t = q[0];
        let i = q[1];
        let v = q[2] as i64;
        if t == 0 {
            if row.insert(i) {
                ans += v * (n - col.len() as i64);
            }
        } else if col.insert(i) {
            ans += v * (n - row.len() as i64);
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        matrix_sum_queries(
            3,
            vec![
                vec![0, 0, 1],
                vec![1, 2, 2],
                vec![0, 2, 3],
                vec![1, 0, 4]
            ]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::matrix_sum_queries;

    #[test]
    fn example_one() {
        assert_eq!(
            matrix_sum_queries(
                3,
                vec![
                    vec![0, 0, 1],
                    vec![1, 2, 2],
                    vec![0, 2, 3],
                    vec![1, 0, 4]
                ]
            ),
            23
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            matrix_sum_queries(
                3,
                vec![
                    vec![0, 0, 4],
                    vec![0, 1, 2],
                    vec![1, 0, 1],
                    vec![0, 2, 3],
                    vec![1, 2, 1]
                ]
            ),
            17
        );
    }
}
