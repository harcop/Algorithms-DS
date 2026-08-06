/// LeetCode #3049 - Earliest Second to Mark Indices II
use std::collections::{BinaryHeap, HashMap};

fn get_second_to_index(nums: &[i32], change_indices: &[i32]) -> HashMap<usize, usize> {
    let mut index_to_second = HashMap::new();
    for (second, &ci) in change_indices.iter().enumerate() {
        let index = (ci - 1) as usize;
        if nums[index] > 0 && !index_to_second.contains_key(&index) {
            index_to_second.insert(index, second);
        }
    }
    index_to_second.into_iter().map(|(k, v)| (v, k)).collect()
}

fn can_mark(nums: &[i32], change_indices: &[i32], max_second: usize) -> bool {
    let second_to_index = get_second_to_index(nums, change_indices);
    let nums_sum: i64 = nums.iter().map(|&x| x as i64).sum();
    let n = nums.len();

    let mut min_heap = BinaryHeap::new();
    let mut marks = 0i64;

    for second in (0..max_second).rev() {
        if let Some(&index) = second_to_index.get(&second) {
            min_heap.push(-(nums[index] as i64));
            if marks == 0 {
                min_heap.pop();
                marks += 1;
            } else {
                marks -= 1;
            }
        } else {
            marks += 1;
        }
    }

    let heap_sum: i64 = min_heap.iter().map(|&x| -x).sum();
    let heap_len = min_heap.len() as i64;
    let decrement_and_mark_cost = (nums_sum - heap_sum) + (n as i64 - heap_len);
    let zero_and_mark_cost = heap_len + heap_len;
    decrement_and_mark_cost + zero_and_mark_cost <= max_second as i64
}

fn earliest_second_to_mark_indices_ii(nums: Vec<i32>, change_indices: Vec<i32>) -> i32 {
    let m = change_indices.len();
    let mut lo = 0i32;
    let mut hi = m as i32;
    let mut ans = m as i32 + 1;

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if can_mark(&nums, &change_indices, mid as usize) {
            ans = mid;
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }

    if ans <= m as i32 {
        ans
    } else {
        -1
    }
}

fn main() {
    println!(
        "{}",
        earliest_second_to_mark_indices_ii(vec![3, 2, 3], vec![1, 3, 2, 2, 2, 2, 3])
    );
}

#[cfg(test)]
mod tests {
    use super::earliest_second_to_mark_indices_ii;

    #[test]
    fn example1() {
        assert_eq!(
            earliest_second_to_mark_indices_ii(vec![3, 2, 3], vec![1, 3, 2, 2, 2, 2, 3]),
            6
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            earliest_second_to_mark_indices_ii(
                vec![0, 0, 1, 2],
                vec![1, 2, 1, 2, 1, 2, 1, 2]
            ),
            7
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            earliest_second_to_mark_indices_ii(vec![1, 2, 3], vec![1, 2, 3]),
            -1
        );
    }
}
