/// LeetCode #3594 - Minimum Time to Transport All Individuals
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, PartialEq)]
struct MinF64(f64);

impl Eq for MinF64 {}

impl PartialOrd for MinF64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for MinF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

fn min_time(n: i32, k: i32, m: i32, time: Vec<i32>, mul: Vec<f64>) -> f64 {
    let n = n as usize;
    let k = k as u32;
    let m = m as usize;
    let full = 1usize << n;
    let mut lookup = vec![0i32; full];
    for mask in 1..full {
        for i in 0..n {
            if mask & (1 << i) != 0 {
                lookup[mask] = lookup[mask].max(time[i]);
            }
        }
    }
    const INF: f64 = 1e18;
    let mut dist = vec![vec![vec![INF; full]; m]; 2];
    dist[0][0][full - 1] = 0.0;
    let mut heap = BinaryHeap::new();
    heap.push((MinF64(0.0), 0usize, 0usize, full - 1));
    while let Some((MinF64(d), r, s, mask)) = heap.pop() {
        if (d - dist[r][s][mask]).abs() > 1e-9 {
            continue;
        }
        if mask == 0 {
            return d;
        }
        if r == 0 {
            let mut sub = mask;
            while sub > 0 {
                if sub.count_ones() <= k {
                    let t = lookup[sub] as f64 * mul[s];
                    let nr = 1;
                    let ns = (s + t.floor() as usize) % m;
                    let nmask = mask ^ sub;
                    let nd = d + t;
                    if nd < dist[nr][ns][nmask] {
                        dist[nr][ns][nmask] = nd;
                        heap.push((MinF64(nd), nr, ns, nmask));
                    }
                }
                sub = (sub - 1) & mask;
            }
        } else {
            for i in 0..n {
                if mask & (1 << i) == 0 {
                    let sub = 1 << i;
                    let t = lookup[sub] as f64 * mul[s];
                    let nr = 0;
                    let ns = (s + t.floor() as usize) % m;
                    let nmask = mask ^ sub;
                    let nd = d + t;
                    if nd < dist[nr][ns][nmask] {
                        dist[nr][ns][nmask] = nd;
                        heap.push((MinF64(nd), nr, ns, nmask));
                    }
                }
            }
        }
    }
    -1.0
}

fn main() {
    println!("{:.5}", min_time(1, 1, 2, vec![5], vec![1.0, 1.3]));
}

#[cfg(test)]
mod tests {
    use super::min_time;

    fn close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-5, "{} vs {}", a, b);
    }

    #[test]
    fn example1() {
        close(min_time(1, 1, 2, vec![5], vec![1.0, 1.3]), 5.0);
    }

    #[test]
    fn example2() {
        close(min_time(3, 2, 3, vec![2, 5, 8], vec![1.0, 1.5, 0.75]), 14.5);
    }

    #[test]
    fn example3() {
        close(min_time(2, 1, 2, vec![10, 10], vec![2.0, 2.0]), -1.0);
    }
}
