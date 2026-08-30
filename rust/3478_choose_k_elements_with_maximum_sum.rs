/// LeetCode #3478 - Choose K Elements With Maximum Sum
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn find_max_sum(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i64> {
    let n = nums1.len();
    let k = k as usize;
    let mut arr: Vec<(i32, usize)> = nums1.iter().copied().enumerate().map(|(i, x)| (x, i)).collect();
    arr.sort_unstable();
    let mut pq: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut s = 0i64;
    let mut j = 0;
    let mut ans = vec![0i64; n];
    for h in 0..n {
        let (x, i) = arr[h];
        while j < h && arr[j].0 < x {
            let y = nums2[arr[j].1];
            pq.push(Reverse(y));
            s += y as i64;
            if pq.len() > k {
                s -= pq.pop().unwrap().0 as i64;
            }
            j += 1;
        }
        ans[i] = s;
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        find_max_sum(vec![4, 2, 1, 5, 3], vec![10, 20, 30, 40, 50], 2)
    );
}

#[cfg(test)]
mod tests {
    use super::find_max_sum;

    #[test]
    fn example1() {
        assert_eq!(
            find_max_sum(vec![4, 2, 1, 5, 3], vec![10, 20, 30, 40, 50], 2),
            vec![80, 30, 0, 80, 50]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            find_max_sum(vec![2, 2, 2, 2], vec![3, 1, 2, 3], 1),
            vec![0, 0, 0, 0]
        );
    }
}
