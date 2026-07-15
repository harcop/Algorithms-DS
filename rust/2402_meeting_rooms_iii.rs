/// LeetCode #2402 - Meeting Rooms III
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    meetings.sort_unstable();
    let mut idle: BinaryHeap<Reverse<usize>> = (0..n).map(Reverse).collect();
    let mut busy: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
    let mut cnt = vec![0usize; n];

    for m in &meetings {
        let s = m[0] as i64;
        let e = m[1] as i64;
        while let Some(&Reverse((end, _))) = busy.peek() {
            if end > s {
                break;
            }
            let Reverse((_, room)) = busy.pop().unwrap();
            idle.push(Reverse(room));
        }
        if let Some(Reverse(room)) = idle.pop() {
            busy.push(Reverse((e, room)));
            cnt[room] += 1;
        } else {
            let Reverse((end, room)) = busy.pop().unwrap();
            busy.push(Reverse((end + e - s, room)));
            cnt[room] += 1;
        }
    }

    let mut ans = 0;
    for i in 1..n {
        if cnt[i] > cnt[ans] {
            ans = i;
        }
    }
    ans as i32
}

fn main() {
    println!(
        "{}",
        most_booked(2, vec![vec![0, 10], vec![1, 5], vec![2, 7], vec![3, 4]])
    );
}

#[cfg(test)]
mod tests {
    use super::most_booked;

    #[test]
    fn example_one() {
        assert_eq!(
            most_booked(2, vec![vec![0, 10], vec![1, 5], vec![2, 7], vec![3, 4]]),
            0
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            most_booked(
                3,
                vec![vec![1, 20], vec![2, 10], vec![3, 5], vec![4, 9], vec![6, 8]]
            ),
            1
        );
    }
}
