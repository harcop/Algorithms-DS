/// LeetCode #632 - Smallest Range Covering Elements from K Lists
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let k = nums.len();
    let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();
    let mut max_v = i32::MIN;
    for i in 0..k {
        let v = nums[i][0];
        heap.push(Reverse((v, i, 0)));
        max_v = max_v.max(v);
    }
    let mut best = (i32::MIN / 2, i32::MAX / 2);
    while let Some(Reverse((min_v, i, j))) = heap.pop() {
        if max_v - min_v < best.1 - best.0 || (max_v - min_v == best.1 - best.0 && min_v < best.0) {
            best = (min_v, max_v);
        }
        if j + 1 == nums[i].len() {
            break;
        }
        let nxt = nums[i][j + 1];
        max_v = max_v.max(nxt);
        heap.push(Reverse((nxt, i, j + 1)));
    }
    vec![best.0, best.1]
}

fn main() {
    let nums = vec![
        vec![4, 10, 15, 24, 26],
        vec![0, 9, 12, 20],
        vec![5, 18, 22, 30],
    ];
    println!("{:?}", smallest_range(nums));
}

#[cfg(test)]
mod tests {
    use super::smallest_range;

    #[test]
    fn example_one() {
        let nums = vec![
            vec![4, 10, 15, 24, 26],
            vec![0, 9, 12, 20],
            vec![5, 18, 22, 30],
        ];
        assert_eq!(smallest_range(nums), vec![20, 24]);
    }

    #[test]
    fn example_two() {
        let nums = vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]];
        assert_eq!(smallest_range(nums), vec![1, 1]);
    }
}
