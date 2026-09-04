/// LeetCode #801 - Minimum Swaps To Make Sequences Increasing
fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let n = nums1.len();
    let inf = i32::MAX / 4;
    let mut keep = 0;
    let mut swap = 1;
    for i in 1..n {
        let mut nk = inf;
        let mut ns = inf;
        if nums1[i] > nums1[i - 1] && nums2[i] > nums2[i - 1] {
            nk = nk.min(keep);
            ns = ns.min(swap + 1);
        }
        if nums1[i] > nums2[i - 1] && nums2[i] > nums1[i - 1] {
            nk = nk.min(swap);
            ns = ns.min(keep + 1);
        }
        if nk == inf && ns == inf {
            return -1;
        }
        keep = nk;
        swap = ns;
    }
    keep.min(swap)
}

fn main() {
    println!("{}", min_swap(vec![1, 3, 5, 4], vec![1, 2, 3, 7]));
}

#[cfg(test)]
mod tests {
    use super::min_swap;

    #[test]
    fn example_one() {
        assert_eq!(min_swap(vec![1, 3, 5, 4], vec![1, 2, 3, 7]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_swap(vec![3, 3, 8, 9, 10], vec![1, 7, 4, 6, 8]), 1);
    }

    #[test]
    fn already_increasing() {
        assert_eq!(min_swap(vec![0, 4, 4, 5], vec![0, 2, 6, 7]), 1);
    }
}
