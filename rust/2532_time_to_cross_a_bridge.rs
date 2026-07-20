/// LeetCode #2532 - Time to Cross a Bridge
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn find_crossing_time(n: i32, k: i32, time: Vec<Vec<i32>>) -> i32 {
    let n = n as i32;
    let k = k as usize;
    let mut ans = 0i32;

    let mut left_bridge: BinaryHeap<(i32, i32)> = BinaryHeap::new();
    let mut right_bridge: BinaryHeap<(i32, i32)> = BinaryHeap::new();
    let mut left_workers: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
    let mut right_workers: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();

    for i in 0..k {
        left_bridge.push((time[i][0] + time[i][2], i as i32));
    }

    let mut boxes = n;
    while boxes > 0 || !right_bridge.is_empty() || !right_workers.is_empty() {
        while let Some(&Reverse((t, i))) = left_workers.peek() {
            if t > ans {
                break;
            }
            left_workers.pop();
            left_bridge.push((time[i as usize][0] + time[i as usize][2], i));
        }
        while let Some(&Reverse((t, i))) = right_workers.peek() {
            if t > ans {
                break;
            }
            right_workers.pop();
            right_bridge.push((time[i as usize][0] + time[i as usize][2], i));
        }

        if let Some((_, i)) = right_bridge.pop() {
            ans += time[i as usize][2];
            left_workers.push(Reverse((ans + time[i as usize][3], i)));
        } else if boxes > 0 {
            if let Some((_, i)) = left_bridge.pop() {
                ans += time[i as usize][0];
                right_workers.push(Reverse((ans + time[i as usize][1], i)));
                boxes -= 1;
            } else {
                let next_left = left_workers
                    .peek()
                    .map(|Reverse((t, _))| *t)
                    .unwrap_or(i32::MAX);
                let next_right = right_workers
                    .peek()
                    .map(|Reverse((t, _))| *t)
                    .unwrap_or(i32::MAX);
                ans = next_left.min(next_right);
            }
        } else {
            let next_right = right_workers
                .peek()
                .map(|Reverse((t, _))| *t)
                .unwrap_or(i32::MAX);
            ans = next_right;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        find_crossing_time(
            1,
            3,
            vec![vec![1, 1, 2, 1], vec![1, 1, 3, 1], vec![1, 1, 4, 1]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::find_crossing_time;

    #[test]
    fn example_one() {
        assert_eq!(
            find_crossing_time(
                1,
                3,
                vec![vec![1, 1, 2, 1], vec![1, 1, 3, 1], vec![1, 1, 4, 1]]
            ),
            6
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_crossing_time(3, 2, vec![vec![1, 9, 1, 8], vec![10, 10, 10, 10]]),
            50
        );
    }
}
