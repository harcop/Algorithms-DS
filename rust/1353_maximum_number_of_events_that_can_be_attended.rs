/// LeetCode #1353 - Maximum Number Of Events That Can Be Attended

use std::collections::BinaryHeap;

fn max_events(events: Vec<Vec<i32>>) -> i32 {
    let mut events = events;
    events.sort_by_key(|e| e[0]);
    let mut heap = BinaryHeap::new();
    let mut i = 0usize;
    let mut day = 0i32;
    let mut attended = 0i32;
    while i < events.len() || !heap.is_empty() {
        if heap.is_empty() {
            day = events[i][0];
        }
        while i < events.len() && events[i][0] == day {
            heap.push(std::cmp::Reverse(events[i][1]));
            i += 1;
        }
        while let Some(&std::cmp::Reverse(end)) = heap.peek() {
            if end < day {
                heap.pop();
            } else {
                break;
            }
        }
        if !heap.is_empty() {
            heap.pop();
            attended += 1;
        }
        day += 1;
    }
    attended
}

fn main() {
    println!("{}", max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4]]));
}

#[cfg(test)]
mod tests {
    use super::max_events;

    #[test]
    fn example_one() {
        assert_eq!(max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4]]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 2]]), 4);
    }
}
