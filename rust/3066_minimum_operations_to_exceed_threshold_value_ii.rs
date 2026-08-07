/// LeetCode #3066 - Minimum Operations to Exceed Threshold Value II
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn min_operations_ii(nums: Vec<i32>, k: i32) -> i32 {
    let mut heap: BinaryHeap<Reverse<i64>> = nums.into_iter().map(|x| Reverse(x as i64)).collect();
    let k = k as i64;
    let mut ops = 0;

    while heap.len() > 1 {
        let Reverse(min) = heap.peek().copied().unwrap();
        if min >= k {
            break;
        }
        let Reverse(x) = heap.pop().unwrap();
        let Reverse(y) = heap.pop().unwrap();
        heap.push(Reverse(x * 2 + y));
        ops += 1;
    }

    ops
}

fn main() {
    println!("{}", min_operations_ii(vec![2, 11, 10, 1, 3], 10));
}

#[cfg(test)]
mod tests {
    use super::min_operations_ii;

    #[test]
    fn example1() {
        assert_eq!(min_operations_ii(vec![2, 11, 10, 1, 3], 10), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations_ii(vec![1, 1, 2, 4, 9], 20), 4);
    }
}
