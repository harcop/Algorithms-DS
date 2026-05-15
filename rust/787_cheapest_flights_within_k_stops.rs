/// LeetCode #787 - Cheapest Flights Within K Stops
use std::collections::VecDeque;

fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    let n = n as usize;
    let mut g: Vec<Vec<(usize, i32)>> = vec![vec![]; n];
    for f in flights {
        g[f[0] as usize].push((f[1] as usize, f[2]));
    }
    let mut dist = vec![i32::MAX; n];
    dist[src as usize] = 0;
    let mut q = VecDeque::new();
    q.push_back((src as usize, 0i32, 0i32));
    let mut best = i32::MAX;
    while let Some((u, d, stops)) = q.pop_front() {
        if u == dst as usize {
            best = best.min(d);
        }
        if stops > k {
            continue;
        }
        for &(v, w) in &g[u] {
            if d + w < dist[v] {
                dist[v] = d + w;
                q.push_back((v, d + w, stops + 1));
            }
        }
    }
    if best == i32::MAX {
        -1
    } else {
        best
    }
}

fn main() {
    let f = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
    println!("{}", find_cheapest_price(3, f, 0, 2, 1));
}

#[cfg(test)]
mod tests {
    use super::find_cheapest_price;

    #[test]
    fn example_one() {
        let f = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        assert_eq!(find_cheapest_price(3, f, 0, 2, 1), 200);
    }
}
