/// LeetCode #1354 - Construct Target Array With Multiple Sums

use std::collections::BinaryHeap;

fn is_possible(target: Vec<i32>) -> bool {
    if target.len() == 1 {
        return target[0] >= 1;
    }
    let mut heap: BinaryHeap<i64> = target.into_iter().map(|x| x as i64).collect();
    let mut sum: i64 = heap.iter().sum();
    while let Some(max) = heap.pop() {
        if max == 1 {
            continue;
        }
        let rest = sum - max;
        if rest < 1 {
            return false;
        }
        let next = max - rest;
        if next < 1 {
            return false;
        }
        sum = rest + next;
        heap.push(next);
    }
    true
}

fn main() {
    println!("{}", is_possible(vec![9]));
}

#[cfg(test)]
mod tests {
    use super::is_possible;

    #[test]
    fn example_one() {
        assert!(is_possible(vec![9]));
    }

    #[test]
    fn example_two() {
        assert!(!is_possible(vec![1, 1, 1, 2]));
    }
}
