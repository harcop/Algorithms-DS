/// LeetCode #3488 - Closest Equal Element Queries
use std::collections::HashMap;

fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let m = n << 1;
    let mut d = vec![m; m];
    let mut left = HashMap::new();
    for i in 0..m {
        let x = nums[i % n];
        if let Some(&p) = left.get(&x) {
            d[i] = d[i].min(i - p);
        }
        left.insert(x, i);
    }
    let mut right = HashMap::new();
    for i in (0..m).rev() {
        let x = nums[i % n];
        if let Some(&p) = right.get(&x) {
            d[i] = d[i].min(p - i);
        }
        right.insert(x, i);
    }
    for i in 0..n {
        d[i] = d[i].min(d[i + n]);
    }
    queries
        .into_iter()
        .map(|i| {
            let i = i as usize;
            if d[i] >= n {
                -1
            } else {
                d[i] as i32
            }
        })
        .collect()
}

fn main() {
    println!("{:?}", solve_queries(vec![1, 3, 1, 4, 1, 3, 2], vec![0, 3, 5]));
}

#[cfg(test)]
mod tests {
    use super::solve_queries;

    #[test]
    fn example1() {
        assert_eq!(
            solve_queries(vec![1, 3, 1, 4, 1, 3, 2], vec![0, 3, 5]),
            vec![2, -1, 3]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            solve_queries(vec![1, 2, 3, 4], vec![0, 1, 2, 3]),
            vec![-1, -1, -1, -1]
        );
    }
}
