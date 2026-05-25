/// LeetCode #1383 - Maximum Performance Of A Team

use std::collections::BinaryHeap;

fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut engineers: Vec<(i32, i32)> = speed.into_iter().zip(efficiency).collect();
    engineers.sort_by_key(|&(_, e)| std::cmp::Reverse(e));
    let mut heap = BinaryHeap::new();
    let mut speed_sum = 0i64;
    let mut best = 0i64;
    for (s, e) in engineers {
        speed_sum += s as i64;
        heap.push(std::cmp::Reverse(s));
        if heap.len() > k as usize {
            speed_sum -= heap.pop().unwrap().0 as i64;
        }
        best = best.max(speed_sum * e as i64);
    }
    (best % MOD) as i32
}

fn main() {
    println!("{}", max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 2));
}

#[cfg(test)]
mod tests {
    use super::max_performance;

    #[test]
    fn example_one() {
        assert_eq!(max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 2), 60);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 3), 68);
    }
}
