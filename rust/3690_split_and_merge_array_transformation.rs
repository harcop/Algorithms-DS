/// LeetCode #3690 - Split and Merge Array Transformation
use std::collections::HashSet;

fn min_split_merge(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let n = nums1.len();
    let target: Vec<i32> = nums2;
    let start = nums1;
    let mut q = vec![start.clone()];
    let mut vis = HashSet::new();
    vis.insert(start);
    let mut ans = 0;
    loop {
        let mut nq = Vec::new();
        for cur in q {
            if cur == target {
                return ans;
            }
            for l in 0..n {
                for r in l..n {
                    let mut remain = Vec::with_capacity(n - (r - l + 1));
                    remain.extend_from_slice(&cur[..l]);
                    remain.extend_from_slice(&cur[r + 1..]);
                    let sub = &cur[l..=r];
                    for i in 0..=remain.len() {
                        let mut nxt = Vec::with_capacity(n);
                        nxt.extend_from_slice(&remain[..i]);
                        nxt.extend_from_slice(sub);
                        nxt.extend_from_slice(&remain[i..]);
                        if vis.insert(nxt.clone()) {
                            nq.push(nxt);
                        }
                    }
                }
            }
        }
        q = nq;
        ans += 1;
    }
}

fn main() {
    println!("{}", min_split_merge(vec![3, 1, 2], vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::min_split_merge;

    #[test]
    fn example1() {
        assert_eq!(min_split_merge(vec![3, 1, 2], vec![1, 2, 3]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_split_merge(vec![1, 1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1, 1]),
            3
        );
    }
}
