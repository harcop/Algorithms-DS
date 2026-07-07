/// LeetCode #2263 - Make Array Non-decreasing or Non-increasing
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn convert_array(nums: Vec<i32>) -> i32 {
    let neg: Vec<i32> = nums.iter().map(|&x| -x).collect();
    cost(&nums).min(cost(&neg))
}

fn cost(nums: &[i32]) -> i32 {
    let mut heap = BinaryHeap::new();
    let mut ans = 0;

    for &num in nums {
        if let Some(&Reverse(top)) = heap.peek() {
            if top < num {
                ans += num - top;
                heap.pop();
                heap.push(Reverse(num));
            }
        }
        heap.push(Reverse(num));
    }

    ans
}

fn main() {
    println!("{}", convert_array(vec![3, 2, 4, 5, 0]));
}

#[cfg(test)]
mod tests {
    use super::convert_array;

    #[test]
    fn example_one() {
        assert_eq!(convert_array(vec![3, 2, 4, 5, 0]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(convert_array(vec![2, 2, 3, 4]), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(convert_array(vec![0]), 0);
    }
}
