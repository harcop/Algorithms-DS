/// LeetCode #3080 - Mark Elements on Array by Performing Queries
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn unmarked_sum_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
    let n = nums.len();
    let mut marked = vec![false; n];
    let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
    for (i, &v) in nums.iter().enumerate() {
        heap.push(Reverse((v, i)));
    }
    let mut unmarked_sum: i64 = nums.iter().map(|&x| x as i64).sum();
    let mut ans = Vec::with_capacity(queries.len());

    for q in queries {
        let idx = q[0] as usize;
        let k = q[1] as usize;
        if !marked[idx] {
            marked[idx] = true;
            unmarked_sum -= nums[idx] as i64;
        }
        let mut marked_count = 0;
        while marked_count < k {
            if let Some(Reverse((v, i))) = heap.pop() {
                if !marked[i] {
                    marked[i] = true;
                    unmarked_sum -= v as i64;
                    marked_count += 1;
                }
            } else {
                break;
            }
        }
        ans.push(unmarked_sum);
    }

    ans
}

fn main() {
    let nums = vec![1, 2, 2, 1, 2, 3, 1];
    let queries = vec![vec![1, 2], vec![3, 3], vec![4, 2]];
    println!("{:?}", unmarked_sum_array(nums, queries));
}

#[cfg(test)]
mod tests {
    use super::unmarked_sum_array;

    #[test]
    fn example1() {
        let nums = vec![1, 2, 2, 1, 2, 3, 1];
        let queries = vec![vec![1, 2], vec![3, 3], vec![4, 2]];
        assert_eq!(unmarked_sum_array(nums, queries), vec![8, 3, 0]);
    }

    #[test]
    fn example2() {
        let nums = vec![1, 4, 2, 3];
        let queries = vec![vec![0, 1]];
        assert_eq!(unmarked_sum_array(nums, queries), vec![7]);
    }
}
