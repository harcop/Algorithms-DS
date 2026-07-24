/// LeetCode #2636 - Promise Pool (JS problem; simulated concurrency pool analogue)
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Run `durations` with concurrency limit `n`.
/// Returns (finish_time per task, pool completion time).
fn promise_pool(durations: &[i32], n: usize) -> (Vec<i32>, i32) {
    let m = durations.len();
    if m == 0 {
        return (vec![], 0);
    }
    let n = n.min(m);
    let mut finish = vec![0; m];
    // min-heap of worker free times
    let mut free: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    for _ in 0..n {
        free.push(Reverse(0));
    }
    for (i, &d) in durations.iter().enumerate() {
        let Reverse(t) = free.pop().unwrap();
        let done = t + d;
        finish[i] = done;
        free.push(Reverse(done));
    }
    let total = *finish.iter().max().unwrap_or(&0);
    (finish, total)
}

fn main() {
    println!("{:?}", promise_pool(&[300, 400, 200], 2));
}

#[cfg(test)]
mod tests {
    use super::promise_pool;

    #[test]
    fn example_one() {
        assert_eq!(
            promise_pool(&[300, 400, 200], 2),
            (vec![300, 400, 500], 500)
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            promise_pool(&[300, 400, 200], 5),
            (vec![300, 400, 200], 400)
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            promise_pool(&[300, 400, 200], 1),
            (vec![300, 700, 900], 900)
        );
    }
}
