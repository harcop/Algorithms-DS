/// LeetCode #870 - Advantage Shuffle
fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut sorted = nums1;
    sorted.sort_unstable();
    let (mut lo, mut hi) = (0, sorted.len() - 1);
    let mut order: Vec<usize> = (0..nums2.len()).collect();
    order.sort_by_key(|&i| nums2[i]);
    let n = nums2.len();
    let mut res = vec![0; n];
    for i in order {
        if sorted[lo] > nums2[i] {
            res[i] = sorted[lo];
            lo += 1;
        } else {
            res[i] = sorted[hi];
            hi -= 1;
        }
    }
    res
}

fn main() {
    println!("{:?}", advantage_count(vec![2, 7, 11, 15], vec![1, 10, 4, 11]));
}

#[cfg(test)]
mod tests {
    use super::advantage_count;

    #[test]
    fn example_one() {
        assert_eq!(
            advantage_count(vec![2, 7, 11, 15], vec![1, 10, 4, 11]),
            vec![2, 11, 7, 15]
        );
    }
}
