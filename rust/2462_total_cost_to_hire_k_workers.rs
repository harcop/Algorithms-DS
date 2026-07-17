/// LeetCode #2462 - Total Cost to Hire K Workers
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
    let n = costs.len();
    let candidates = candidates as usize;
    let mut left = 0usize;
    let mut right = n - 1;
    let mut left_heap = BinaryHeap::new();
    let mut right_heap = BinaryHeap::new();

    for _ in 0..candidates {
        if left <= right {
            left_heap.push(Reverse((costs[left], left)));
            left += 1;
        }
    }
    for _ in 0..candidates {
        if left <= right {
            right_heap.push(Reverse((costs[right], right)));
            if right == 0 {
                break;
            }
            right -= 1;
        }
    }

    let mut answer = 0i64;
    for _ in 0..k {
        let left_best = left_heap.peek().map(|entry| entry.0);
        let right_best = right_heap.peek().map(|entry| entry.0);
        let take_left = match (left_best, right_best) {
            (Some(left_worker), Some(right_worker)) => left_worker <= right_worker,
            (Some(_), None) => true,
            (None, Some(_)) => false,
            (None, None) => unreachable!(),
        };

        if take_left {
            let Reverse((cost, _)) = left_heap.pop().unwrap();
            answer += cost as i64;
            if left <= right {
                left_heap.push(Reverse((costs[left], left)));
                left += 1;
            }
        } else {
            let Reverse((cost, _)) = right_heap.pop().unwrap();
            answer += cost as i64;
            if left <= right {
                right_heap.push(Reverse((costs[right], right)));
                if right > 0 {
                    right -= 1;
                } else {
                    left = 1;
                }
            }
        }
    }

    answer
}

fn main() {
    println!("{}", total_cost(vec![17, 12, 10, 2, 7, 2, 11, 20, 8], 3, 4));
}

#[cfg(test)]
mod tests {
    use super::total_cost;

    #[test]
    fn example_one() {
        assert_eq!(total_cost(vec![17, 12, 10, 2, 7, 2, 11, 20, 8], 3, 4), 11);
    }

    #[test]
    fn example_two() {
        assert_eq!(total_cost(vec![1, 2, 4, 1], 3, 3), 4);
    }
}
