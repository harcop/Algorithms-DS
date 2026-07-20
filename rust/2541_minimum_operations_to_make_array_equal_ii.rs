/// LeetCode #2541 - Minimum Operations to Make Array Equal II
fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    let mut a = 0i64;
    let mut b = 0i64;
    for (&x, &y) in nums1.iter().zip(nums2.iter()) {
        if x == y {
            continue;
        }
        if k == 0 || (x - y) % k != 0 {
            return -1;
        }
        let t = (x - y) / k;
        if t < 0 {
            a += (-t) as i64;
        } else {
            b += t as i64;
        }
    }
    if a == b {
        a
    } else {
        -1
    }
}

fn main() {
    println!("{}", min_operations(vec![4, 3, 1, 4], vec![1, 3, 7, 1], 3));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example_one() {
        assert_eq!(
            min_operations(vec![4, 3, 1, 4], vec![1, 3, 7, 1], 3),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            min_operations(vec![3, 8, 5, 2], vec![2, 4, 1, 6], 1),
            -1
        );
    }
}
