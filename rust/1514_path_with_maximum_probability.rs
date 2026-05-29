/// LeetCode #1514 - Path With Maximum Probability
use std::collections::BinaryHeap;
use std::cmp::Ordering;

fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start_node: i32, end_node: i32) -> f64 {
    let n = n as usize;
    let mut g: Vec<Vec<(usize, f64)>> = vec![vec![]; n];
    for (e, &p) in edges.iter().zip(succ_prob.iter()) {
        let u = e[0] as usize;
        let v = e[1] as usize;
        g[u].push((v, p));
        g[v].push((u, p));
    }
    let mut dist = vec![0.0f64; n];
    dist[start_node as usize] = 1.0;
    let mut heap = BinaryHeap::new();
    heap.push((OrderedFloat(dist[start_node as usize]), start_node as usize));
    while let Some((OrderedFloat(d), u)) = heap.pop() {
        if d < dist[u] {
            continue;
        }
        for &(v, p) in &g[u] {
            let nd = d * p;
            if nd > dist[v] {
                dist[v] = nd;
                heap.push((OrderedFloat(nd), v));
            }
        }
    }
    dist[end_node as usize]
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
struct OrderedFloat(f64);
impl Eq for OrderedFloat {}
impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(Ordering::Equal)
    }
}

fn main() {
    println!("{}", max_probability(3, vec![vec![0, 1], vec![1, 2], vec![0, 2]], vec![0.5, 0.5, 0.2], 0, 2));
}

#[cfg(test)]
mod tests {
    use super::max_probability;

    #[test]
    fn example_one() {
        let p = max_probability(3, vec![vec![0, 1], vec![1, 2], vec![0, 2]], vec![0.5, 0.5, 0.2], 0, 2);
        assert!((p - 0.25).abs() < 1e-9);
    }

    #[test]
    fn example_two() {
        let p = max_probability(3, vec![vec![0, 1], vec![1, 2], vec![0, 2]], vec![0.5, 0.5, 0.3], 0, 2);
        assert!((p - 0.3).abs() < 1e-9);
    }
}
