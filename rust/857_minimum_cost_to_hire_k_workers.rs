/// LeetCode #857 - Minimum Cost to Hire K Workers
fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
    let k = k as usize;
    let mut workers: Vec<(f64, i32)> = wage
        .into_iter()
        .zip(quality)
        .map(|(w, q)| (w as f64 / q as f64, q))
        .collect();
    workers.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut heap = std::collections::BinaryHeap::new();
    let mut sum_q = 0i64;
    let mut ans = f64::MAX;
    for (ratio, q) in workers {
        heap.push(q);
        sum_q += q as i64;
        if heap.len() > k {
            sum_q -= heap.pop().unwrap() as i64;
        }
        if heap.len() == k {
            ans = ans.min(ratio * sum_q as f64);
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        mincost_to_hire_workers(vec![10, 20, 5], vec![70, 50, 30], 2)
    );
}

#[cfg(test)]
mod tests {
    use super::mincost_to_hire_workers;

    #[test]
    fn example_one() {
        assert!((mincost_to_hire_workers(vec![10, 20, 5], vec![70, 50, 30], 2) - 105.0).abs() < 1e-5);
    }
}
