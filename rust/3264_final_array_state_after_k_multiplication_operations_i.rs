/// LeetCode #3264 - Final Array State After K Multiplication Operations I
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
    let mut pq: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
    for (i, &x) in nums.iter().enumerate() {
        pq.push(Reverse((x, i)));
    }
    for _ in 0..k {
        let Reverse((_, i)) = pq.pop().unwrap();
        nums[i] *= multiplier;
        pq.push(Reverse((nums[i], i)));
    }
    nums
}

fn main() {
    println!("{:?}", get_final_state(vec![2, 1, 3, 5, 6], 5, 2));
}

#[cfg(test)]
mod tests {
    use super::get_final_state;

    #[test]
    fn example1() {
        assert_eq!(get_final_state(vec![2, 1, 3, 5, 6], 5, 2), vec![8, 4, 6, 5, 6]);
    }

    #[test]
    fn example2() {
        assert_eq!(get_final_state(vec![1, 2], 3, 4), vec![16, 8]);
    }
}
