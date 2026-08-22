/// LeetCode #3362 - Zero Array Transformation III
use std::collections::BinaryHeap;

fn max_removal(nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> i32 {
    queries.sort_by_key(|q| q[0]);
    let n = nums.len();
    let mut pq = BinaryHeap::new();
    let mut d = vec![0i32; n + 1];
    let mut s = 0;
    let mut j = 0;
    for i in 0..n {
        s += d[i];
        while j < queries.len() && queries[j][0] <= i as i32 {
            pq.push(queries[j][1]);
            j += 1;
        }
        while s < nums[i] && pq.peek().is_some_and(|&r| r >= i as i32) {
            s += 1;
            let r = pq.pop().unwrap();
            d[r as usize + 1] -= 1;
        }
        if s < nums[i] {
            return -1;
        }
    }
    pq.len() as i32
}

fn main() {
    println!(
        "{}",
        max_removal(vec![2, 0, 2], vec![vec![0, 2], vec![0, 2], vec![1, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::max_removal;

    #[test]
    fn example1() {
        assert_eq!(
            max_removal(vec![2, 0, 2], vec![vec![0, 2], vec![0, 2], vec![1, 1]]),
            1
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_removal(
                vec![1, 1, 1, 1],
                vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![1, 2]]
            ),
            2
        );
    }

    #[test]
    fn example3() {
        assert_eq!(max_removal(vec![1, 2, 3, 4], vec![vec![0, 3]]), -1);
    }
}
