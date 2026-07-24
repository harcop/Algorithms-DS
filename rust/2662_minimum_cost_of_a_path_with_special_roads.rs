/// LeetCode #2662 - Minimum Cost of a Path With Special Roads
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

fn dist(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn minimum_cost(start: Vec<i32>, target: Vec<i32>, special_roads: Vec<Vec<i32>>) -> i32 {
    let mut ans = i32::MAX;
    let n = 1_000_000i64;
    let mut pq: BinaryHeap<Reverse<(i32, i32, i32)>> = BinaryHeap::new();
    pq.push(Reverse((0, start[0], start[1])));
    let mut vis = HashSet::new();
    while let Some(Reverse((d, x, y))) = pq.pop() {
        let key = x as i64 * n + y as i64;
        if vis.contains(&key) {
            continue;
        }
        vis.insert(key);
        ans = ans.min(d + dist(x, y, target[0], target[1]));
        for r in &special_roads {
            let (x1, y1, x2, y2, cost) = (r[0], r[1], r[2], r[3], r[4]);
            pq.push(Reverse((d + dist(x, y, x1, y1) + cost, x2, y2)));
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        minimum_cost(
            vec![1, 1],
            vec![4, 5],
            vec![vec![1, 2, 3, 3, 2], vec![3, 4, 4, 5, 1]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_cost;

    #[test]
    fn example_one() {
        assert_eq!(
            minimum_cost(
                vec![1, 1],
                vec![4, 5],
                vec![vec![1, 2, 3, 3, 2], vec![3, 4, 4, 5, 1]]
            ),
            5
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            minimum_cost(
                vec![3, 2],
                vec![5, 7],
                vec![
                    vec![5, 7, 3, 2, 1],
                    vec![3, 2, 3, 4, 4],
                    vec![3, 3, 5, 5, 5],
                    vec![3, 4, 5, 6, 6]
                ]
            ),
            7
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            minimum_cost(
                vec![1, 1],
                vec![10, 4],
                vec![
                    vec![4, 2, 1, 1, 3],
                    vec![1, 2, 7, 4, 4],
                    vec![10, 3, 6, 1, 2],
                    vec![6, 1, 1, 2, 3]
                ]
            ),
            8
        );
    }
}
