/// LeetCode #2918 - Minimum Equal Sum of Two Arrays After Replacing Zeros
fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
    let summarize = |nums: &[i32]| -> (i64, i64) {
        let mut s = 0i64;
        let mut zeros = 0i64;
        for &x in nums {
            if x == 0 {
                zeros += 1;
                s += 1;
            } else {
                s += x as i64;
            }
        }
        (s, zeros)
    };

    let (s1, z1) = summarize(&nums1);
    let (s2, z2) = summarize(&nums2);
    if s1 == s2 {
        s1
    } else if s1 < s2 {
        if z1 == 0 {
            -1
        } else {
            s2
        }
    } else if z2 == 0 {
        -1
    } else {
        s1
    }
}

fn main() {
    println!("{}", min_sum(vec![3, 2, 0, 1, 0], vec![6, 5, 0]));
}

#[cfg(test)]
mod tests {
    use super::min_sum;

    #[test]
    fn example_one() {
        assert_eq!(min_sum(vec![3, 2, 0, 1, 0], vec![6, 5, 0]), 12);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_sum(vec![2, 0, 2, 0], vec![1, 4]), -1);
    }
}
