/// LeetCode #1942 - The Number of the Smallest Unoccupied Chair
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
    let mut events: Vec<Vec<i32>> = times
        .into_iter()
        .enumerate()
        .map(|(i, mut t)| {
            t.push(i as i32);
            t
        })
        .collect();
    events.sort_unstable();
    let n = events.len();
    let mut idle: BinaryHeap<Reverse<i32>> = (0..n as i32).map(Reverse).collect();
    let mut busy: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();

    for evt in events {
        let arrival = evt[0];
        let leaving = evt[1];
        let i = evt[2];
        while let Some(&Reverse((t, _))) = busy.peek() {
            if t <= arrival {
                let Reverse((_, chair)) = busy.pop().unwrap();
                idle.push(Reverse(chair));
            } else {
                break;
            }
        }
        let Reverse(chair) = idle.pop().unwrap();
        if i == target_friend {
            return chair;
        }
        busy.push(Reverse((leaving, chair)));
    }
    0
}

fn main() {
    println!(
        "{}",
        smallest_chair(vec![vec![1, 4], vec![2, 3], vec![4, 6]], 1)
    );
}

#[cfg(test)]
mod tests {
    use super::smallest_chair;

    #[test]
    fn example_one() {
        assert_eq!(
            smallest_chair(vec![vec![1, 4], vec![2, 3], vec![4, 6]], 1),
            1
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            smallest_chair(vec![vec![3, 10], vec![1, 5], vec![2, 6]], 0),
            2
        );
    }
}
