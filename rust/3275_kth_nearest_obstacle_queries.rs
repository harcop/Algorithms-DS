/// LeetCode #3275 - K-th Nearest Obstacle Queries
use std::collections::BinaryHeap;

fn results_array(queries: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut pq = BinaryHeap::new();
    let mut ans = Vec::with_capacity(queries.len());
    for (i, q) in queries.iter().enumerate() {
        pq.push(q[0].abs() + q[1].abs());
        if i >= k {
            pq.pop();
        }
        ans.push(if i >= k - 1 {
            *pq.peek().unwrap()
        } else {
            -1
        });
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        results_array(vec![vec![1, 2], vec![3, 4], vec![2, 3], vec![-3, 0]], 2)
    );
}

#[cfg(test)]
mod tests {
    use super::results_array;

    #[test]
    fn example1() {
        assert_eq!(
            results_array(vec![vec![1, 2], vec![3, 4], vec![2, 3], vec![-3, 0]], 2),
            vec![-1, 7, 5, 3]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            results_array(vec![vec![5, 5], vec![4, 4], vec![3, 3]], 1),
            vec![10, 8, 6]
        );
    }
}
