/// LeetCode #253 - Meeting Rooms II
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
    let mut v = intervals;
    v.sort_by_key(|x| x[0]);
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut rooms = 0;
    for it in v {
        while let Some(&Reverse(e)) = heap.peek() {
            if e <= it[0] {
                heap.pop();
            } else {
                break;
            }
        }
        heap.push(Reverse(it[1]));
        rooms = rooms.max(heap.len());
    }
    rooms as i32
}

fn main() {
    println!("{}", min_meeting_rooms(vec![vec![0, 30], vec![5, 10], vec![15, 20]]));
}

#[cfg(test)]
mod tests {
    use super::min_meeting_rooms;

    #[test]
    fn example_one() {
        assert_eq!(
            min_meeting_rooms(vec![vec![0, 30], vec![5, 10], vec![15, 20]]),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(min_meeting_rooms(vec![vec![7, 10], vec![2, 4]]), 1);
    }
}
