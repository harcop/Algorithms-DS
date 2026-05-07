/// LeetCode #373 - K Smallest Pairs with Sums (min-heap)
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let k = k as usize;
    if nums1.is_empty() || nums2.is_empty() || k == 0 {
        return vec![];
    }
    let mut h: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();
    for i in 0..nums1.len().min(k) {
        h.push(Reverse((nums1[i] + nums2[0], i, 0)));
    }
    let mut out = vec![];
    while out.len() < k {
        let Some(Reverse((_s, i, j))) = h.pop() else {
            break;
        };
        out.push(vec![nums1[i], nums2[j]]);
        if j + 1 < nums2.len() {
            h.push(Reverse((nums1[i] + nums2[j + 1], i, j + 1)));
        }
    }
    out
}

fn main() {
    println!("{:?}", k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        assert_eq!(
            k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3),
            vec![vec![1, 2], vec![1, 4], vec![1, 6]]
        );
    }
}
