/// LeetCode #1834 - Single-Threaded CPU
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
    let n = tasks.len();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by_key(|&i| tasks[i][0]);

    let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
    let mut ans = Vec::with_capacity(n);
    let mut time = 0i64;
    let mut idx = 0usize;

    while idx < n || !heap.is_empty() {
        while idx < n && tasks[order[idx]][0] as i64 <= time {
            let i = order[idx];
            heap.push(Reverse((tasks[i][1], i)));
            idx += 1;
        }
        if heap.is_empty() {
            time = tasks[order[idx]][0] as i64;
            continue;
        }
        let Reverse((proc, i)) = heap.pop().unwrap();
        ans.push(i as i32);
        time += proc as i64;
    }
    ans
}

fn main() {
    println!("{:?}", get_order(vec![vec![1, 2], vec![2, 4], vec![3, 2], vec![4, 1]]));
}

#[cfg(test)]
mod tests {
    use super::get_order;

    #[test]
    fn example_one() {
        assert_eq!(
            get_order(vec![vec![1, 2], vec![2, 4], vec![3, 2], vec![4, 1]]),
            vec![0, 2, 3, 1]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            get_order(vec![
                vec![7, 10],
                vec![7, 12],
                vec![7, 5],
                vec![7, 4],
                vec![7, 2]
            ]),
            vec![4, 3, 2, 0, 1]
        );
    }
}
