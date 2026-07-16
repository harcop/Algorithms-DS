/// LeetCode #2406 - Divide Intervals Into Minimum Number of Groups
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn min_groups(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_unstable();
    let mut heap = BinaryHeap::new();
    let mut ans = 0;

    for interval in intervals {
        let start = interval[0];
        let end = interval[1];
        while let Some(&Reverse(finish)) = heap.peek() {
            if finish >= start {
                break;
            }
            heap.pop();
        }
        heap.push(Reverse(end));
        ans = ans.max(heap.len() as i32);
    }

    ans
}

fn main() {
    println!("{}", min_groups(vec![vec![5, 10], vec![6, 8], vec![1, 5], vec![2, 3], vec![1, 10]]));
}

#[cfg(test)]
mod tests {
    use super::min_groups;

    #[test]
    fn example_one() {
        assert_eq!(
            min_groups(vec![vec![5, 10], vec![6, 8], vec![1, 5], vec![2, 3], vec![1, 10]]),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(min_groups(vec![vec![1, 3], vec![5, 6], vec![8, 10], vec![11, 13]]), 1);
    }
}
