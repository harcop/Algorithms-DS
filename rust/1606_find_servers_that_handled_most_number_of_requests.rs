/// LeetCode #1606 - Find Servers That Handled Most Number Of Requests
use std::cmp::Reverse;
use std::collections::{BinaryHeap, BTreeSet};

fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
    let k = k as i32;
    let mut free: BTreeSet<i32> = (0..k).collect();
    let mut busy: BinaryHeap<Reverse<(i64, i32)>> = BinaryHeap::new();
    let mut cnt = vec![0i32; k as usize];
    for i in 0..arrival.len() {
        let t = arrival[i] as i64;
        while let Some(&Reverse((end, s))) = busy.peek() {
            if end > t {
                break;
            }
            busy.pop();
            free.insert(s);
        }
        let want = (i as i32) % k;
        let server = if let Some(&s) = free.range(want..).next() {
            free.remove(&s);
            s
        } else if let Some(&s) = free.iter().next() {
            free.remove(&s);
            s
        } else {
            let Reverse((end, s)) = busy.pop().unwrap();
            cnt[s as usize] += 1;
            let start = end.max(t);
            busy.push(Reverse((start + load[i] as i64, s)));
            continue;
        };
        cnt[server as usize] += 1;
        busy.push(Reverse((t + load[i] as i64, server)));
    }
    let mx = *cnt.iter().max().unwrap_or(&0);
    if mx == 0 {
        return vec![];
    }
    (0..k).filter(|&i| cnt[i as usize] == mx).map(|i| i).collect()
}

fn main() {
    println!("{:?}", busiest_servers(3, vec![1, 2, 3, 4, 5], vec![5, 2, 3, 3, 3]));
}

#[cfg(test)]
mod tests {
    use super::busiest_servers;

    #[test]
    fn example_one() {
        assert_eq!(busiest_servers(3, vec![1, 2, 3, 4, 5], vec![5, 2, 3, 3, 3]), vec![0, 1]);
    }

    #[test]
    fn example_two() {
        assert_eq!(busiest_servers(3, vec![1, 2, 3, 4], vec![1, 2, 1, 2]), vec![0]);
    }
}
