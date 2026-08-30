/// LeetCode #3476 - Maximize Profit from Task Assignment
use std::collections::{BinaryHeap, HashMap};

fn max_profit(workers: Vec<i32>, tasks: Vec<Vec<i32>>) -> i64 {
    let mut d: HashMap<i32, BinaryHeap<i32>> = HashMap::new();
    for t in &tasks {
        d.entry(t[0]).or_default().push(t[1]);
    }
    let mut ans = 0i64;
    for skill in workers {
        if let Some(heap) = d.get_mut(&skill) {
            if let Some(p) = heap.pop() {
                ans += p as i64;
            }
        }
    }
    let mx = d.values().filter_map(|h| h.peek()).copied().max().unwrap_or(0);
    ans + mx as i64
}

fn main() {
    println!(
        "{}",
        max_profit(vec![1, 2, 3, 4, 5], vec![vec![1, 100], vec![2, 400], vec![3, 100], vec![3, 400]])
    );
}

#[cfg(test)]
mod tests {
    use super::max_profit;

    #[test]
    fn example1() {
        assert_eq!(
            max_profit(
                vec![1, 2, 3, 4, 5],
                vec![vec![1, 100], vec![2, 400], vec![3, 100], vec![3, 400]]
            ),
            1000
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_profit(vec![10, 10000, 100000000], vec![vec![1, 100]]),
            100
        );
    }

    #[test]
    fn example3() {
        assert_eq!(max_profit(vec![7], vec![vec![3, 3], vec![3, 3]]), 3);
    }
}
