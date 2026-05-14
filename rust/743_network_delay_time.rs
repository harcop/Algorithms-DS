/// LeetCode #743 - Network Delay Time
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    let n = n as usize;
    let k = (k - 1) as usize;
    let mut g: HashMap<usize, Vec<(usize, i32)>> = HashMap::new();
    for t in times {
        let u = (t[0] - 1) as usize;
        let v = (t[1] - 1) as usize;
        g.entry(u).or_default().push((v, t[2]));
    }
    let mut dist = vec![i32::MAX; n];
    dist[k] = 0;
    let mut pq: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
    pq.push(Reverse((0, k)));
    while let Some(Reverse((d, u))) = pq.pop() {
        if d > dist[u] {
            continue;
        }
        if let Some(nei) = g.get(&u) {
            for &(v, w) in nei {
                let nd = d + w;
                if nd < dist[v] {
                    dist[v] = nd;
                    pq.push(Reverse((nd, v)));
                }
            }
        }
    }
    let mx = dist.iter().copied().max().unwrap();
    if mx == i32::MAX {
        -1
    } else {
        mx
    }
}

fn main() {
    let t = vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]];
    println!("{}", network_delay_time(t, 4, 2));
}

#[cfg(test)]
mod tests {
    use super::network_delay_time;

    #[test]
    fn example_one() {
        let t = vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]];
        assert_eq!(network_delay_time(t, 4, 2), 2);
    }
}
